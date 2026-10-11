use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use caudal_south_api::Op;
use caudal_types::{PaymentId, PaymentState, ReasonCode, Usd};
use jiff::{SignedDuration, Timestamp};
use serde::Serialize;

use crate::commands::{Line, Tone, clock, name_of};
use crate::sandbox::{INSTITUTIONS, LLAVES, Sandbox, money, start_of_day};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    Single,
    Payroll,
    CutOff,
    National,
}

impl Scenario {
    pub const ALL: [Self; 4] = [Self::Single, Self::Payroll, Self::CutOff, Self::National];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Payroll => "payroll",
            Self::CutOff => "cutoff",
            Self::National => "national",
        }
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::Single => "One payment, banco-alfa to coop-beta, then a settlement",
            Self::Payroll => "Payroll day at coop-beta, a segment-3 cooperativa",
            Self::CutOff => "Cut-off rush: the last thirty minutes before 17:00",
            Self::National => "One day of national traffic",
        }
    }
}

impl FromStr for Scenario {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|x| x.name() == s)
            .ok_or_else(|| format!("no scenario {s:?}; try single, payroll, cutoff or national"))
    }
}

impl fmt::Display for Scenario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

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

    fn amount(&mut self, max_cents: u64) -> Usd {
        let u = self.unit();
        Usd::from_cents(300 + (u * u * u * (max_cents - 300) as f64) as u64)
    }
}

/// Share of background traffic and largest amount in cents, sized to each ceiling.
fn profile(institution: &str) -> (u32, u64) {
    match institution {
        "banco-alfa" => (60, 150_000),
        "mutual-delta" => (26, 100_000),
        "coop-beta" => (9, 25_000),
        _ => (5, 10_000),
    }
}

fn pick_source(rng: &mut Rng) -> &'static str {
    let weights = INSTITUTIONS.map(|i| profile(i.id).0);
    INSTITUTIONS[rng.weighted(&weights)].id
}

fn payable(llave: &crate::sandbox::LlaveSeed) -> bool {
    !llave.account.ends_with("99") && !llave.account.ends_with("98")
}

fn pick_beneficiary(rng: &mut Rng) -> &'static str {
    let to = pick_source(rng);
    let llaves: Vec<_> = LLAVES.iter().filter(|l| l.institution == to && payable(l)).collect();
    llaves[rng.below(llaves.len())].id
}

fn at(hh: i64, mm: i64) -> Timestamp {
    start_of_day() + SignedDuration::from_hours(hh - 8) + SignedDuration::from_mins(mm)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Background,
    Salary,
    Retry,
}

#[derive(Debug, Clone)]
enum Step {
    Pay { from: &'static str, llave: &'static str, amount: Usd, kind: Kind },
    Settle,
    Snapshot,
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

#[derive(Debug, Clone, Default, Serialize)]
pub struct Payroll {
    pub items: usize,
    pub paid_first_try: usize,
    pub blocked_am04: usize,
    pub paid_after_settlement: usize,
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
    pub scenario: &'static str,
    pub title: &'static str,
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
    pub payroll: Option<Payroll>,
    pub stats: Vec<OpReport>,
    pub reserve_to_post_ms: Option<[f64; 3]>,
    pub compute_ms: f64,
    pub wall_ms: f64,
}

pub const DISCLAIMER: &str = "Demostración con datos ficticios. Proyecto Caudal es independiente y no está afiliado al Banco Central del Ecuador.";

fn plan(scenario: Scenario, rng: &mut Rng) -> Vec<(Timestamp, Step)> {
    let mut steps = Vec::new();
    let background =
        |steps: &mut Vec<(Timestamp, Step)>, rng: &mut Rng, when: &mut dyn FnMut(&mut Rng) -> Timestamp, n: usize| {
            for _ in 0..n {
                let at = when(rng);
                let source = pick_source(rng);
                let llave = pick_beneficiary(rng);
                let amount = rng.amount(profile(source).1);
                steps.push((at, Step::Pay { from: source, llave, amount, kind: Kind::Background }));
            }
        };
    match scenario {
        Scenario::Single => {
            steps.push((
                at(9, 0),
                Step::Pay {
                    from: "banco-alfa",
                    llave: "llave-01",
                    amount: Usd::from_cents(2_540),
                    kind: Kind::Background,
                },
            ));
            steps.push((at(9, 1), Step::Settle));
        }
        Scenario::Payroll => {
            let staff = ["llave-02", "llave-04", "llave-05"];
            for i in 0..300 {
                let when = at(8, 0) + SignedDuration::from_secs_f64(i as f64 * 18.0 + rng.unit() * 10.0);
                let amount = Usd::from_cents(18_000 + rng.next() % 34_000);
                steps.push((
                    when,
                    Step::Pay { from: "coop-beta", llave: staff[i % staff.len()], amount, kind: Kind::Salary },
                ));
            }
            background(&mut steps, rng, &mut |r| at(8, 0) + SignedDuration::from_secs_f64(r.unit() * 14_400.0), 600);
            steps.extend([(at(11, 0), Step::Settle), (at(15, 0), Step::Settle)]);
            steps.extend((0..=28).map(|q| (at(8, 0) + SignedDuration::from_mins(q * 15), Step::Snapshot)));
        }
        Scenario::CutOff => {
            background(
                &mut steps,
                rng,
                &mut |r| at(16, 30) + SignedDuration::from_secs_f64(r.unit().sqrt() * 1_800.0),
                3_000,
            );
            steps.push((at(17, 30), Step::Settle));
            steps.extend((0..=12).map(|q| (at(16, 30) + SignedDuration::from_mins(q * 5), Step::Snapshot)));
        }
        Scenario::National => {
            const HOURLY: [u32; 9] = [6, 9, 10, 9, 8, 8, 9, 10, 9];
            background(
                &mut steps,
                rng,
                &mut |r| at(8 + r.weighted(&HOURLY) as i64, 0) + SignedDuration::from_secs_f64(r.unit() * 3_600.0),
                20_000,
            );
            let staff = ["llave-02", "llave-04", "llave-05"];
            for i in 0..300 {
                let when = at(9, 0) + SignedDuration::from_secs_f64(i as f64 * 12.0);
                let amount = Usd::from_cents(18_000 + rng.next() % 34_000);
                steps.push((
                    when,
                    Step::Pay { from: "coop-beta", llave: staff[i % staff.len()], amount, kind: Kind::Salary },
                ));
            }
            steps.extend([(at(11, 0), Step::Settle), (at(15, 0), Step::Settle), (at(17, 30), Step::Settle)]);
            steps.extend((0..=38).map(|q| (at(8, 0) + SignedDuration::from_mins(q * 15), Step::Snapshot)));
        }
    }
    steps.sort_by_key(|(t, _)| *t);
    steps
}

pub fn run(scenario: Scenario, seed: u64) -> Report {
    run_with(scenario, seed).0
}

pub fn run_with(scenario: Scenario, seed: u64) -> (Report, Sandbox) {
    let started = Instant::now();
    let mut rng = Rng(seed);
    let mut sandbox = Sandbox::empty(start_of_day(), SignedDuration::from_millis(4));
    if scenario == Scenario::National {
        sandbox.network_mut().record_hops(false);
    }
    let mut queue: BTreeMap<(Timestamp, usize), Step> =
        plan(scenario, &mut rng).into_iter().enumerate().map(|(i, (t, s))| ((t, i), s)).collect();
    let mut next = queue.len();
    let mut salaries: Vec<PaymentId> = Vec::new();
    let mut retries: Vec<PaymentId> = Vec::new();
    let mut retried: HashSet<PaymentId> = HashSet::new();
    let mut settlements = Vec::new();
    let mut timeline = Vec::new();
    while let Some(((when, _), step)) = queue.pop_first() {
        sandbox.network_mut().run_until(when);
        match step {
            Step::Pay { from, llave, amount, kind } => {
                let order = sandbox.order(from, llave, amount).expect("scenarios use seeded ids");
                match kind {
                    Kind::Salary => salaries.push(order.id.clone()),
                    Kind::Retry => retries.push(order.id.clone()),
                    Kind::Background => {}
                }
                let now = sandbox.now();
                sandbox.network_mut().schedule_payment(now, order);
            }
            Step::Settle => {
                sandbox.network_mut().run_until(when + SignedDuration::from_secs(25));
                let s = sandbox.network_mut().settle().expect("the in-memory ledger never fails");
                settlements.push(SettlementReport {
                    at: clock(s.at),
                    payments: s.malla.payments.len(),
                    gross: money(i128::from(s.malla.gross().cents())),
                    net: money(i128::from(s.malla.total().cents())),
                    nets: s.malla.nets.iter().map(|(b, n)| (name_of(*b), money(*n))).collect(),
                });
                let blocked: Vec<_> = salaries
                    .iter()
                    .chain(&retries)
                    .filter(|id| !retried.contains(*id))
                    .filter_map(|id| sandbox.network().row(id).filter(|r| r.reason == Some(ReasonCode::Am04)).cloned())
                    .collect();
                for (i, row) in blocked.into_iter().enumerate() {
                    let llave = LLAVES
                        .iter()
                        .find(|l| crate::sandbox::institution(l.institution).is_some_and(|x| x.bic == row.to.as_str()))
                        .map_or("llave-02", |l| l.id);
                    retried.insert(row.id);
                    let at = s.at + SignedDuration::from_secs(60) + SignedDuration::from_millis(i as i64 * 400);
                    queue.insert(
                        (at, next),
                        Step::Pay { from: "coop-beta", llave, amount: row.amount, kind: Kind::Retry },
                    );
                    next += 1;
                }
            }
            Step::Snapshot => timeline.push(Snapshot {
                at: clock(when),
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
            }),
        }
    }
    sandbox.network_mut().run_until_idle();
    let report = summarize(scenario, seed, &sandbox, &salaries, &retries, settlements, timeline, started);
    (report, sandbox)
}

#[allow(clippy::too_many_arguments)]
fn summarize(
    scenario: Scenario,
    seed: u64,
    sandbox: &Sandbox,
    salaries: &[PaymentId],
    retries: &[PaymentId],
    settlements: Vec<SettlementReport>,
    timeline: Vec<Snapshot>,
    started: Instant,
) -> Report {
    let net = sandbox.network();
    let rows = net.rows();
    let mut rejected = BTreeMap::new();
    let mut accepted_value = 0u64;
    for r in rows {
        match r.state {
            PaymentState::Rjct => {
                *rejected
                    .entry(r.reason.map_or("RJCT".into(), |c| c.to_string()))
                    .or_insert_with(BTreeMap::new)
                    .entry(name_of(r.from))
                    .or_insert(0) += 1;
            }
            PaymentState::Acsp | PaymentState::Acsc => accepted_value += r.amount.cents(),
            _ => {}
        }
    }
    let paid =
        |id: &&PaymentId| net.row(id).is_some_and(|r| matches!(r.state, PaymentState::Acsp | PaymentState::Acsc));
    let payroll = (!salaries.is_empty()).then(|| Payroll {
        items: salaries.len(),
        paid_first_try: salaries.iter().filter(paid).count(),
        blocked_am04: salaries
            .iter()
            .filter(|id| net.row(id).is_some_and(|r| r.reason == Some(ReasonCode::Am04)))
            .count(),
        paid_after_settlement: retries.iter().filter(paid).count(),
        last_paid_at: salaries
            .iter()
            .chain(retries)
            .filter(paid)
            .filter_map(|id| net.row(id))
            .map(|r| r.updated)
            .max()
            .map(clock),
    });
    let stats = net.stats();
    let us = |d: std::time::Duration| (d.as_nanos() as f64 / 10.0).round() / 100.0;
    Report {
        scenario: scenario.name(),
        title: scenario.title(),
        disclaimer: DISCLAIMER,
        seed,
        date: start_of_day().strftime("%Y-%m-%d").to_string(),
        payments: rows.len(),
        accepted: rows.iter().filter(|r| matches!(r.state, PaymentState::Acsp | PaymentState::Acsc)).count(),
        settled: rows.iter().filter(|r| r.state == PaymentState::Acsc).count(),
        intra_red: rows
            .iter()
            .filter(|r| {
                let red = |b| crate::sandbox::institution_by_bic(b).map(|i| i.red);
                red(r.from) == red(r.to)
            })
            .count(),
        rejected,
        por_conciliar: rows.iter().filter(|r| r.por_conciliar).count(),
        value_accepted: money(i128::from(accepted_value)),
        settlements,
        timeline,
        payroll,
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

pub fn render(r: &Report) -> Vec<Line> {
    let mut out = Vec::new();
    let mut push = |tone, text: String| out.push(Line { tone, text });
    push(Tone::Heading, format!("Scenario {}: {}", r.scenario, r.title));
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
    if let Some(p) = &r.payroll {
        push(Tone::Heading, "coop-beta payroll".into());
        push(
            Tone::Plain,
            format!(
                "  items {}   paid first try {}   blocked AM04 {}   paid after a settlement {}",
                p.items, p.paid_first_try, p.blocked_am04, p.paid_after_settlement
            ),
        );
        if let Some(t) = &p.last_paid_at {
            push(Tone::Plain, format!("  last salary paid at {t}"));
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

    #[test]
    fn the_single_scenario_settles_one_payment() {
        let r = run(Scenario::Single, 1);
        assert_eq!((r.payments, r.settled), (1, 1));
    }

    #[test]
    fn payroll_hits_the_ceiling_and_recovers_after_settlement() {
        let r = run(Scenario::Payroll, 7);
        let p = r.payroll.unwrap();
        assert_eq!(p.items, 300);
        assert!(p.blocked_am04 > 0, "a segment-3 payroll exceeds the ceiling between settlements");
        assert!(p.paid_after_settlement > 0, "a settlement restores headroom");
        assert_eq!(p.paid_first_try + p.blocked_am04, p.items);
    }

    #[test]
    fn scenarios_replay_exactly() {
        let a = run(Scenario::CutOff, 3);
        let b = run(Scenario::CutOff, 3);
        assert_eq!(
            (a.accepted, &a.rejected, a.value_accepted.clone()),
            (b.accepted, &b.rejected, b.value_accepted.clone())
        );
    }
}
