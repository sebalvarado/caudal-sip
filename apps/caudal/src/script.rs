use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;

use caudal_south_api::Op;
use caudal_types::{PaymentId, PaymentState, ReasonCode, Usd};
use jiff::{SignedDuration, Timestamp};
use serde::Serialize;
use web_time::Instant;

use crate::commands::{self, Command, Line, Tone, clock, name_of, state_label};
use crate::sandbox::{INSTITUTIONS, LLAVES, Sandbox, institution, institution_by_bic, money, start_of_day};

pub const DISCLAIMER: &str = "Demostración con datos ficticios. Proyecto Caudal es independiente y no está afiliado al Banco Central del Ecuador.";

pub const HELP: &[(&str, &str)] = &[
    ("seed <n>", "seed for random traffic and amounts (default 7)"),
    ("ceiling <institution> <amount>", "replace an institution's ceiling before the day starts"),
    ("at <HH:MM[:SS]> [statement]", "move to that time; the lines after it follow from there"),
    ("wait <duration>", "move forward, e.g. wait 30s, wait 5m, wait 1h"),
    ("pay <from> <llave> <amount>", "send a payment and wait for its outcome"),
    (
        "repeat <n> every <duration>: pay <from> <llaves> <amount>",
        "n payments without waiting; llaves a,b,c; amount min..max",
    ),
    ("traffic <n> until <HH:MM>", "n random payments between institutions, sized to each ceiling"),
    ("retry", "send again the script's payments rejected with AM04"),
    ("settle, network, show, resolve, llaves, fault", "as in the terminal"),
];

const MAX_PAYMENTS: usize = 50_000;
const RECORD_HOPS_UP_TO: usize = 5_000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

/// SplitMix64: enough for reproducible synthetic traffic, and no dependency.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn weighted(&mut self, weights: &[u32]) -> usize {
        let total: u32 = weights.iter().sum();
        let mut pick = (self.next() % u64::from(total)) as u32;
        for (i, w) in weights.iter().enumerate() {
            if pick < *w {
                return i;
            }
            pick -= w;
        }
        weights.len() - 1
    }

    fn between(&mut self, min: Usd, max: Usd) -> Usd {
        Usd::from_cents(min.cents() + self.next() % (max.cents() - min.cents() + 1))
    }

    /// Mostly small amounts with a long tail, the shape of retail payments.
    fn retail(&mut self, max_cents: u64) -> Usd {
        let u = self.unit();
        Usd::from_cents(300 + (u * u * u * (max_cents - 300) as f64) as u64)
    }
}

/// Share of random traffic and largest amount in cents, sized to each ceiling.
fn profile(institution: &str) -> (u32, u64) {
    match institution {
        "banco-alfa" => (60, 150_000),
        "mutual-delta" => (26, 100_000),
        "coop-beta" => (9, 25_000),
        _ => (5, 10_000),
    }
}

fn pick_institution(rng: &mut Rng) -> &'static str {
    let weights = INSTITUTIONS.map(|i| profile(i.id).0);
    INSTITUTIONS[rng.weighted(&weights)].id
}

fn pick_llave(rng: &mut Rng) -> &'static str {
    let to = pick_institution(rng);
    let payable: Vec<_> = LLAVES
        .iter()
        .filter(|l| l.institution == to && !l.account.ends_with("99") && !l.account.ends_with("98"))
        .collect();
    payable[rng.below(payable.len())].id
}

#[derive(Debug, Clone, Copy)]
enum Amount {
    Fixed(Usd),
    Range(Usd, Usd),
}

impl Amount {
    fn draw(self, rng: &mut Rng) -> Usd {
        match self {
            Self::Fixed(a) => a,
            Self::Range(min, max) => rng.between(min, max),
        }
    }
}

#[derive(Debug, Clone)]
enum Statement {
    Pay { from: &'static str, llaves: Vec<&'static str>, amount: Amount, repeat: Option<(usize, SignedDuration)> },
    Traffic { n: usize, until: Timestamp },
    Settle,
    Retry,
    Inspect(Command),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Traffic,
    Tracked,
    Retry,
}

#[derive(Debug, Clone)]
enum Action {
    Pay { from: &'static str, llave: &'static str, amount: Usd, wait: bool, kind: Kind },
    Settle,
    Retry,
    Inspect(Command),
    Snapshot,
}

struct Plan {
    title: String,
    seed: u64,
    ceilings: Vec<(&'static str, Usd)>,
    actions: Vec<(Timestamp, Action)>,
    payments: usize,
}

fn time(word: &str) -> Result<Timestamp, String> {
    let parts: Vec<&str> = word.split(':').collect();
    let num = |s: &str, max: i64| s.parse::<i64>().ok().filter(|n| (0..=max).contains(n) && s.len() == 2);
    let (h, m, s) = match parts.as_slice() {
        [h, m] => (num(h, 23), num(m, 59), Some(0)),
        [h, m, s] => (num(h, 23), num(m, 59), num(s, 59)),
        _ => (None, None, None),
    };
    let (Some(h), Some(m), Some(s)) = (h, m, s) else {
        return Err(format!("time {word:?}: use HH:MM or HH:MM:SS"));
    };
    if h < 8 {
        return Err(format!("time {word}: the day starts at 08:00"));
    }
    Ok(start_of_day() + SignedDuration::from_secs((h - 8) * 3_600 + m * 60 + s))
}

fn duration(word: &str) -> Result<SignedDuration, String> {
    let split = word.find(|c: char| !c.is_ascii_digit()).unwrap_or(word.len());
    let (n, unit) = word.split_at(split);
    let n: i64 = n.parse().map_err(|_| format!("duration {word:?}: e.g. 400ms, 30s, 5m, 1h"))?;
    match unit {
        "ms" => Ok(SignedDuration::from_millis(n)),
        "s" => Ok(SignedDuration::from_secs(n)),
        "m" => Ok(SignedDuration::from_mins(n)),
        "h" => Ok(SignedDuration::from_hours(n)),
        _ => Err(format!("duration {word:?}: e.g. 400ms, 30s, 5m, 1h")),
    }
}

fn usd(word: &str) -> Result<Usd, String> {
    word.parse::<Usd>()
        .ok()
        .filter(|a| *a > Usd::ZERO)
        .ok_or_else(|| format!("amount {word:?}: a positive number with at most two decimals"))
}

fn amount(word: &str) -> Result<Amount, String> {
    match word.split_once("..") {
        None => usd(word).map(Amount::Fixed),
        Some((a, b)) => {
            let (min, max) = (usd(a)?, usd(b)?);
            if min > max {
                return Err(format!("amount {word}: the minimum is above the maximum"));
            }
            Ok(Amount::Range(min, max))
        }
    }
}

fn from(word: &str) -> Result<&'static str, String> {
    institution(word)
        .map(|i| i.id)
        .ok_or_else(|| format!("no institution {word:?}; try banco-alfa, mutual-delta, coop-beta or fintech-gamma"))
}

fn llave(word: &str) -> Result<&'static str, String> {
    LLAVES
        .iter()
        .find(|l| l.id == word || l.value == word)
        .map(|l| l.id)
        .ok_or_else(|| format!("no llave {word:?}; the seeded ones are llave-01 to llave-08"))
}

fn count(word: &str) -> Result<usize, String> {
    word.parse()
        .ok()
        .filter(|n| (1..=MAX_PAYMENTS).contains(n))
        .ok_or_else(|| format!("{word:?}: a count from 1 to 50000"))
}

fn statement(words: &[&str], at: Timestamp) -> Result<Statement, String> {
    Ok(match words {
        ["pay", f, l, a] => {
            if l.contains(',') {
                return Err("a single pay takes one llave; use repeat for several".into());
            }
            Statement::Pay { from: from(f)?, llaves: vec![llave(l)?], amount: amount(a)?, repeat: None }
        }
        ["repeat", n, "every", every, rest @ ..] => {
            let (every, rest) = match (every.strip_suffix(':'), rest) {
                (Some(e), rest) => (e, rest),
                (None, [":", rest @ ..]) => (*every, rest),
                _ => return Err("repeat <n> every <duration>: pay ...".into()),
            };
            let ["pay", f, ls, a] = rest else {
                return Err("repeat <n> every <duration>: pay <from> <llaves> <amount>".into());
            };
            let llaves = ls.split(',').map(llave).collect::<Result<Vec<_>, _>>()?;
            Statement::Pay { from: from(f)?, llaves, amount: amount(a)?, repeat: Some((count(n)?, duration(every)?)) }
        }
        ["traffic", n, "until", t] => {
            let until = time(t)?;
            if until <= at {
                return Err(format!("traffic until {t}: that is not after the current time"));
            }
            Statement::Traffic { n: count(n)?, until }
        }
        ["settle"] => Statement::Settle,
        ["retry"] => Statement::Retry,
        [first, ..] if ["network", "headroom", "positions", "show", "resolve", "llaves", "fault"].contains(first) => {
            Statement::Inspect(commands::parse(&words.join(" "))?)
        }
        [first, ..] => return Err(format!("unknown statement {first:?}; see the reference")),
        [] => unreachable!("blank lines are skipped"),
    })
}

fn plan(source: &str, seed: Option<u64>) -> Result<Plan, ParseError> {
    let mut title = String::new();
    let mut script_seed = 7;
    let mut ceilings = Vec::new();
    let mut statements: Vec<(Timestamp, usize, Statement)> = Vec::new();
    let mut at = start_of_day();
    for (i, raw) in source.lines().enumerate() {
        let line = i + 1;
        let err = |message: String| ParseError { line, message };
        if title.is_empty()
            && statements.is_empty()
            && let Some(comment) = raw.trim().strip_prefix('#')
        {
            title = comment.trim().to_string();
        }
        let code = raw.split('#').next().unwrap_or_default();
        let mut words: Vec<&str> = code.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        match words.as_slice() {
            ["seed", n] | ["ceiling", n, _] if !statements.is_empty() => {
                let _ = n;
                return Err(err(format!("{} goes before the first statement", words[0])));
            }
            ["seed", n] => {
                script_seed = n.parse().map_err(|_| err(format!("seed {n:?}: a whole number")))?;
                continue;
            }
            ["ceiling", who, a] => {
                ceilings.push((from(who).map_err(err)?, usd(a).map_err(err)?));
                continue;
            }
            ["at", t, ..] => {
                let next = time(t).map_err(err)?;
                if next < at {
                    return Err(err(format!("at {t}: the script is already at {}", clock(at))));
                }
                at = next;
                words.drain(..2);
            }
            ["wait", d] => {
                at += duration(d).map_err(err)?;
                continue;
            }
            _ => {}
        }
        if words.is_empty() {
            continue;
        }
        statements.push((at, line, statement(&words, at).map_err(err)?));
    }
    let payments: usize = statements
        .iter()
        .map(|(_, _, s)| match s {
            Statement::Pay { repeat: Some((n, _)), .. } | Statement::Traffic { n, .. } => *n,
            Statement::Pay { .. } => 1,
            _ => 0,
        })
        .sum();
    if payments > MAX_PAYMENTS {
        let line = statements.last().map_or(1, |(_, l, _)| *l);
        return Err(ParseError {
            line,
            message: format!("{payments} payments; a script sends at most {MAX_PAYMENTS}"),
        });
    }
    let seed = seed.unwrap_or(script_seed);
    let mut rng = Rng(seed);
    let mut actions = Vec::new();
    for (at, _, s) in statements {
        match s {
            Statement::Pay { from, llaves, amount, repeat: None } => {
                let pay = Action::Pay {
                    from,
                    llave: llaves[0],
                    amount: amount.draw(&mut rng),
                    wait: true,
                    kind: Kind::Tracked,
                };
                actions.push((at, pay));
            }
            Statement::Pay { from, llaves, amount, repeat: Some((n, every)) } => {
                for i in 0..n {
                    let pay = Action::Pay {
                        from,
                        llave: llaves[i % llaves.len()],
                        amount: amount.draw(&mut rng),
                        wait: false,
                        kind: Kind::Tracked,
                    };
                    actions.push((at + every * i as i32, pay));
                }
            }
            Statement::Traffic { n, until } => {
                let span = until.duration_since(at).as_secs_f64();
                for _ in 0..n {
                    let when = at + SignedDuration::from_secs_f64(rng.unit() * span);
                    let source = pick_institution(&mut rng);
                    let llave = pick_llave(&mut rng);
                    let amount = rng.retail(profile(source).1);
                    actions.push((when, Action::Pay { from: source, llave, amount, wait: false, kind: Kind::Traffic }));
                }
            }
            Statement::Settle => actions.push((at, Action::Settle)),
            Statement::Retry => actions.push((at, Action::Retry)),
            Statement::Inspect(c) => actions.push((at, Action::Inspect(c))),
        }
    }
    let last = actions.iter().map(|(t, _)| *t).max().unwrap_or(start_of_day());
    let mut snapshot = start_of_day();
    while snapshot <= last {
        actions.push((snapshot, Action::Snapshot));
        snapshot += SignedDuration::from_mins(15);
    }
    Ok(Plan { title, seed, ceilings, actions, payments })
}

#[derive(Debug, Clone, Serialize)]
pub struct LogLine {
    pub at: String,
    pub tone: Tone,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaymentLine {
    pub id: String,
    pub at: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub state: String,
    pub retry: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SettlementReport {
    pub at: String,
    pub payments: usize,
    pub gross: String,
    pub net: String,
    pub nets: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InstitutionSnapshot {
    pub id: &'static str,
    pub used_percent: f64,
    pub light: &'static str,
    pub available: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Snapshot {
    pub at: String,
    pub institutions: Vec<InstitutionSnapshot>,
}

/// The payments the script names itself, as opposed to random traffic.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Tracked {
    pub items: usize,
    pub paid_first_try: usize,
    pub blocked_am04: usize,
    pub paid_after_retry: usize,
    pub last_paid_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpReport {
    pub op: &'static str,
    pub count: usize,
    pub p50_us: f64,
    pub p95_us: f64,
    pub p99_us: f64,
    pub per_second: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub title: String,
    pub disclaimer: &'static str,
    pub seed: u64,
    pub date: String,
    pub payments: usize,
    pub accepted: usize,
    pub settled: usize,
    pub intra_red: usize,
    pub rejected: BTreeMap<String, BTreeMap<String, usize>>,
    pub por_conciliar: usize,
    pub value_accepted: String,
    pub settlements: Vec<SettlementReport>,
    pub timeline: Vec<Snapshot>,
    pub tracked: Option<Tracked>,
    pub stats: Vec<OpReport>,
    pub reserve_to_post_ms: Option<[f64; 3]>,
    pub compute_ms: f64,
    pub wall_ms: f64,
}

pub struct Run {
    pub report: Report,
    pub log: Vec<LogLine>,
    pub payments: Vec<PaymentLine>,
    pub sandbox: Sandbox,
}

const PAYMENT_LINES: usize = 1_000;

/// Parses without running; returns how many payments the script would send.
pub fn check(source: &str) -> Result<usize, ParseError> {
    plan(source, None).map(|p| p.payments)
}

/// Runs a script on a fresh network. `seed` overrides the script's own.
pub fn run(source: &str, seed: Option<u64>) -> Result<Run, ParseError> {
    run_with_progress(source, seed, |_| {})
}

/// Like [`run`], reporting the fraction of the simulated day done at every snapshot.
pub fn run_with_progress(source: &str, seed: Option<u64>, mut progress: impl FnMut(f64)) -> Result<Run, ParseError> {
    let started = Instant::now();
    let plan = plan(source, seed)?;
    let first = start_of_day();
    let last = plan.actions.iter().map(|(t, _)| *t).max().unwrap_or(first);
    let span = last.duration_since(first).as_secs_f64().max(1.0);
    let mut sandbox = Sandbox::with_ceilings(start_of_day(), SignedDuration::from_millis(4), &plan.ceilings);
    sandbox.network_mut().record_hops(plan.payments <= RECORD_HOPS_UP_TO);
    let mut queue: BTreeMap<(Timestamp, usize), Action> =
        plan.actions.into_iter().enumerate().map(|(i, (t, a))| ((t, i), a)).collect();
    let mut next = queue.len();
    let mut log = Vec::new();
    let mut sent: HashMap<PaymentId, (&'static str, &'static str, Usd)> = HashMap::new();
    let mut tracked: Vec<PaymentId> = Vec::new();
    let mut retries: Vec<PaymentId> = Vec::new();
    let mut retried: HashSet<PaymentId> = HashSet::new();
    let mut settlements = Vec::new();
    let mut timeline = Vec::new();
    while let Some(((when, _), action)) = queue.pop_first() {
        if when > sandbox.now() {
            sandbox.network_mut().run_until(when);
        }
        let now = clock(sandbox.now());
        match action {
            Action::Pay { from, llave, amount, wait, kind } => {
                let id = if wait {
                    let id = sandbox.pay(from, llave, amount).expect("ids are checked when parsing");
                    let row = sandbox.network().row(&id).expect("just sent");
                    let tone = commands::state_tone(row);
                    let text = format!(
                        "pay {from} → {} USD {}  {}",
                        name_of(row.to),
                        money(i128::from(amount.cents())),
                        state_label(row)
                    );
                    log.push(LogLine { at: now, tone, text });
                    id
                } else {
                    let order = sandbox.order(from, llave, amount).expect("ids are checked when parsing");
                    let id = order.id.clone();
                    let at = sandbox.now();
                    sandbox.network_mut().schedule_payment(at, order);
                    id
                };
                match kind {
                    Kind::Traffic => {}
                    Kind::Tracked => tracked.push(id.clone()),
                    Kind::Retry => retries.push(id.clone()),
                }
                if kind != Kind::Traffic {
                    sent.insert(id, (from, llave, amount));
                }
            }
            Action::Settle => {
                let s = sandbox.network_mut().settle().expect("the in-memory ledger never fails");
                let gross = money(i128::from(s.malla.gross().cents()));
                let net = money(i128::from(s.malla.total().cents()));
                let n = s.malla.payments.len();
                log.push(LogLine {
                    at: now,
                    tone: Tone::Good,
                    text: format!("settle  {n} payments  gross USD {gross}  net USD {net}"),
                });
                settlements.push(SettlementReport {
                    at: clock(s.at),
                    payments: n,
                    gross,
                    net,
                    nets: s.malla.nets.iter().map(|(b, n)| (name_of(*b), money(*n))).collect(),
                });
            }
            Action::Retry => {
                let blocked: Vec<PaymentId> = tracked
                    .iter()
                    .chain(&retries)
                    .filter(|id| !retried.contains(*id))
                    .filter(|id| sandbox.network().row(id).is_some_and(|r| r.reason == Some(ReasonCode::Am04)))
                    .cloned()
                    .collect();
                for (i, id) in blocked.iter().enumerate() {
                    retried.insert(id.clone());
                    let (from, llave, amount) = sent[id];
                    let at = sandbox.now() + SignedDuration::from_millis(i as i64 * 400);
                    queue.insert((at, next), Action::Pay { from, llave, amount, wait: false, kind: Kind::Retry });
                    next += 1;
                }
                let tone = if blocked.is_empty() { Tone::Dim } else { Tone::Warn };
                log.push(LogLine {
                    at: now,
                    tone,
                    text: format!("retry  {} payments rejected with AM04 sent again", blocked.len()),
                });
            }
            Action::Inspect(command) => {
                let out = commands::execute(&mut sandbox, command);
                log.extend(out.lines.into_iter().map(|l| LogLine { at: now.clone(), tone: l.tone, text: l.text }));
            }
            Action::Snapshot => {
                timeline.push(snapshot(&sandbox, when));
                progress((when.duration_since(first).as_secs_f64() / span).clamp(0.0, 1.0));
            }
        }
    }
    sandbox.network_mut().run_until_idle();
    let payments = payment_lines(&sandbox, &tracked, &retries);
    let report = summarize(&plan.title, plan.seed, &sandbox, &tracked, &retries, settlements, timeline, started);
    Ok(Run { report, log, payments, sandbox })
}

fn snapshot(sandbox: &Sandbox, at: Timestamp) -> Snapshot {
    Snapshot {
        at: clock(at),
        institutions: sandbox
            .institutions()
            .into_iter()
            .map(|v| InstitutionSnapshot {
                id: v.id,
                used_percent: v.used_percent(),
                light: v.light.label(),
                available: money(v.available),
            })
            .collect(),
    }
}

fn payment_lines(sandbox: &Sandbox, tracked: &[PaymentId], retries: &[PaymentId]) -> Vec<PaymentLine> {
    let net = sandbox.network();
    let retry: HashSet<&PaymentId> = retries.iter().collect();
    let mut lines: Vec<PaymentLine> = tracked
        .iter()
        .chain(retries)
        .take(PAYMENT_LINES)
        .filter_map(|id| net.row(id))
        .map(|r| PaymentLine {
            id: r.id.to_string(),
            at: clock(r.created),
            from: name_of(r.from),
            to: name_of(r.to),
            amount: money(i128::from(r.amount.cents())),
            state: state_label(r),
            retry: retry.contains(&r.id),
        })
        .collect();
    lines.sort_by(|a, b| a.at.cmp(&b.at));
    lines
}

#[allow(clippy::too_many_arguments)]
fn summarize(
    title: &str,
    seed: u64,
    sandbox: &Sandbox,
    tracked: &[PaymentId],
    retries: &[PaymentId],
    settlements: Vec<SettlementReport>,
    timeline: Vec<Snapshot>,
    started: Instant,
) -> Report {
    let net = sandbox.network();
    let rows = net.rows();
    let mut rejected: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut accepted_value = 0u64;
    for r in rows {
        match r.state {
            PaymentState::Rjct => {
                *rejected
                    .entry(r.reason.map_or("RJCT".into(), |c| c.to_string()))
                    .or_default()
                    .entry(name_of(r.from))
                    .or_insert(0) += 1;
            }
            PaymentState::Acsp | PaymentState::Acsc => accepted_value += r.amount.cents(),
            _ => {}
        }
    }
    let paid =
        |id: &&PaymentId| net.row(id).is_some_and(|r| matches!(r.state, PaymentState::Acsp | PaymentState::Acsc));
    let tracked_report = (!tracked.is_empty()).then(|| Tracked {
        items: tracked.len(),
        paid_first_try: tracked.iter().filter(paid).count(),
        blocked_am04: tracked
            .iter()
            .filter(|id| net.row(id).is_some_and(|r| r.reason == Some(ReasonCode::Am04)))
            .count(),
        paid_after_retry: retries.iter().filter(paid).count(),
        last_paid_at: tracked
            .iter()
            .chain(retries)
            .filter(paid)
            .filter_map(|id| net.row(id))
            .map(|r| r.created)
            .max()
            .map(clock),
    });
    let red = |b| institution_by_bic(b).map(|i| i.red);
    let stats = net.stats();
    let us = |d: std::time::Duration| (d.as_nanos() as f64 / 10.0).round() / 100.0;
    Report {
        title: title.to_string(),
        disclaimer: DISCLAIMER,
        seed,
        date: start_of_day().strftime("%Y-%m-%d").to_string(),
        payments: rows.len(),
        accepted: rows.iter().filter(|r| matches!(r.state, PaymentState::Acsp | PaymentState::Acsc)).count(),
        settled: rows.iter().filter(|r| r.state == PaymentState::Acsc).count(),
        intra_red: rows.iter().filter(|r| red(r.from) == red(r.to)).count(),
        rejected,
        por_conciliar: rows.iter().filter(|r| r.por_conciliar).count(),
        value_accepted: money(i128::from(accepted_value)),
        settlements,
        timeline,
        tracked: tracked_report,
        stats: Op::ALL
            .into_iter()
            .filter_map(|op| {
                stats.summary(op).map(|s| OpReport {
                    op: op.label(),
                    count: s.count,
                    p50_us: us(s.p50),
                    p95_us: us(s.p95),
                    p99_us: us(s.p99),
                    per_second: s.per_second().round(),
                })
            })
            .collect(),
        reserve_to_post_ms: stats.reserve_to_post().map(|s| [s.p50, s.p95, s.p99].map(|d| d.as_secs_f64() * 1_000.0)),
        compute_ms: stats.compute().as_secs_f64() * 1_000.0,
        wall_ms: started.elapsed().as_secs_f64() * 1_000.0,
    }
}

pub fn help() -> Vec<Line> {
    let mut lines = vec![Line { tone: Tone::Heading, text: "Scenario scripts".into() }];
    lines.extend(HELP.iter().map(|(usage, what)| Line { tone: Tone::Plain, text: format!("  {usage:<58} {what}") }));
    lines
}

pub fn render(r: &Report) -> Vec<Line> {
    let mut out = Vec::new();
    let mut push = |tone, text: String| out.push(Line { tone, text });
    if !r.title.is_empty() {
        push(Tone::Heading, r.title.clone());
    }
    push(
        Tone::Plain,
        format!(
            "  payments {}   accepted {}   settled {}   intra-red {}   por conciliar {}",
            r.payments, r.accepted, r.settled, r.intra_red, r.por_conciliar
        ),
    );
    push(Tone::Plain, format!("  value accepted  USD {}", r.value_accepted));
    for (code, by) in &r.rejected {
        let detail: Vec<String> = by.iter().map(|(who, n)| format!("{who} {n}")).collect();
        push(Tone::Bad, format!("  rejected {code}  {}  ({})", by.values().sum::<usize>(), detail.join(", ")));
    }
    if let Some(t) = &r.tracked {
        push(Tone::Heading, "Scripted payments".into());
        push(
            Tone::Plain,
            format!(
                "  items {}   paid first try {}   blocked AM04 {}   paid after a retry {}",
                t.items, t.paid_first_try, t.blocked_am04, t.paid_after_retry
            ),
        );
        if let Some(at) = &t.last_paid_at {
            push(Tone::Plain, format!("  last one paid at {at}"));
        }
    }
    for s in &r.settlements {
        push(
            Tone::Good,
            format!("  settlement {}  {} payments  gross USD {}  net USD {}", s.at, s.payments, s.gross, s.net),
        );
    }
    push(Tone::Heading, "Compute per hop (a floor: no network, TLS or core banking)".into());
    for s in &r.stats {
        push(
            Tone::Plain,
            format!(
                "  {:<10} n={:<8} p50 {:>8.2}µs  p95 {:>8.2}µs  p99 {:>8.2}µs  {:>10.0}/s",
                s.op, s.count, s.p50_us, s.p95_us, s.p99_us, s.per_second
            ),
        );
    }
    if let Some([p50, p95, p99]) = r.reserve_to_post_ms {
        push(Tone::Plain, format!("  reserve→post (simulated)  p50 {p50:.0} ms  p95 {p95:.0} ms  p99 {p99:.0} ms"));
    }
    push(Tone::Dim, format!("  {} ms of compute, {:.0} ms wall clock", r.compute_ms.round(), r.wall_ms));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fails(source: &str) -> ParseError {
        run(source, None).err().expect("a parse error")
    }

    #[test]
    fn a_single_pay_waits_for_its_outcome_and_settles() {
        let r = run("at 09:00 pay coop-beta llave-02 25.40\nsettle\n", None).unwrap();
        assert!(r.log[0].text.contains("ACSP"), "{:?}", r.log);
        assert_eq!(r.log[0].at, "09:00:00.000");
        assert_eq!((r.report.payments, r.report.settled), (1, 1));
        assert_eq!(r.payments[0].state, "ACSC");
    }

    #[test]
    fn repeat_spreads_payments_and_cycles_llaves() {
        let r = run("seed 1\nat 09:00 repeat 4 every 1m: pay banco-alfa llave-01,llave-04 10..20\n", None).unwrap();
        let at: Vec<&str> = r.payments.iter().map(|p| p.at.as_str()).collect();
        assert_eq!(at, ["09:00:00.000", "09:01:00.000", "09:02:00.000", "09:03:00.000"]);
        let to: Vec<&str> = r.payments.iter().map(|p| p.to.as_str()).collect();
        assert_eq!(to, ["coop-beta", "mutual-delta", "coop-beta", "mutual-delta"]);
        assert!(r.payments.iter().all(|p| ("10.00"..="20.00").contains(&p.amount.as_str())));
    }

    #[test]
    fn a_lower_ceiling_blocks_and_a_retry_after_settlement_pays() {
        let source = "ceiling coop-beta 100\nat 09:00 repeat 3 every 1s: pay coop-beta llave-02 60\nat 10:00 settle\nwait 1m\nretry\n";
        let t = run(source, None).unwrap().report.tracked.unwrap();
        assert_eq!((t.items, t.paid_first_try, t.blocked_am04, t.paid_after_retry), (3, 1, 2, 1));
    }

    #[test]
    fn errors_point_at_the_line() {
        assert_eq!(fails("pay coop-beta llave-02 25.40\npay nobody llave-02 1").line, 2);
        assert!(fails("pay coop-beta llave-99 1").message.contains("llave-01 to llave-08"));
        assert!(fails("at 10:00 settle\nat 09:00 settle").message.contains("already at 10:00"));
        assert!(fails("settle\nseed 3").message.contains("before the first statement"));
        assert!(fails("at 07:00 settle").message.contains("starts at 08:00"));
        assert!(fails("traffic 60000 until 17:00").message.contains("1 to 50000"));
        assert!(fails("scenario payroll").message.contains("unknown statement"));
    }

    #[test]
    fn the_same_seed_replays_the_same_day() {
        let source = "at 08:00 traffic 300 until 09:00\nat 09:00 settle\n";
        let (a, b) = (run(source, None).unwrap().report, run(source, None).unwrap().report);
        assert_eq!((a.accepted, &a.value_accepted, &a.rejected), (b.accepted, &b.value_accepted, &b.rejected));
        assert_ne!(run(source, Some(8)).unwrap().report.value_accepted, b.value_accepted);
    }
}
