use std::sync::Arc;

use caudal_ledger::{EntryQuery, Leg};
use caudal_payments::Message;
use caudal_south_api::{Hop, Row};
use caudal_types::{Bic, PaymentState, Usd, to_scheme_time};

use crate::sandbox::{LLAVES, Sandbox, bic, institution_by_bic, money};
use crate::scenarios::Scenario;
use crate::script;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tone {
    Plain,
    Heading,
    Good,
    Bad,
    Warn,
    Dim,
    Xml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub tone: Tone,
    pub text: String,
}

#[derive(Debug, Default)]
pub struct Output {
    pub lines: Vec<Line>,
    pub quit: bool,
}

impl Output {
    fn push(&mut self, tone: Tone, text: impl Into<String>) {
        self.lines.push(Line { tone, text: text.into() });
    }

    pub fn plain(&self) -> String {
        self.lines.iter().map(|l| format!("{}\n", l.text)).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    Pay { from: String, llave: String, amount: Usd },
    Resolve(String),
    Llaves,
    Settle,
    Network,
    Show { id: Option<String>, xml: bool },
    Scenario(Scenario),
    Fault(Option<u8>),
    Quit,
}

pub const HELP: &[(&str, &str)] = &[
    ("pay <from> <llave> <amount>", "send a payment, e.g. pay banco-alfa llave-01 25.40"),
    ("resolve <llave>", "resolve a llave to its masked name and institution"),
    ("llaves", "list the seeded llaves"),
    ("settle", "close and settle the current malla"),
    ("network", "headroom, traffic light and net position per institution"),
    ("show [<payment-id>|last] [xml]", "a payment's hops, states, postings and, with xml, every message"),
    ("scenario <single|payroll|cutoff|national>", "run a built-in scenario script"),
    ("fault <1|2|3|4|off>", "inject one of the catálogo's four exceptions into the next payments"),
    ("quit", "leave"),
];

pub fn help() -> Vec<Line> {
    let mut lines = vec![Line { tone: Tone::Heading, text: "Commands".into() }];
    lines.extend(HELP.iter().map(|(usage, what)| Line { tone: Tone::Plain, text: format!("  {usage:<44} {what}") }));
    lines
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<&str> = line.split_whitespace().collect();
    Ok(match words.as_slice() {
        [] | ["help"] => Command::Help,
        ["pay", from, llave, amount] => Command::Pay {
            from: (*from).into(),
            llave: (*llave).into(),
            amount: amount.parse().map_err(|_| format!("amount {amount:?}: digits with at most two decimals"))?,
        },
        ["resolve", llave] => Command::Resolve((*llave).into()),
        ["llaves"] => Command::Llaves,
        ["settle"] => Command::Settle,
        ["network" | "headroom" | "positions"] => Command::Network,
        ["show"] => Command::Show { id: None, xml: false },
        ["show", "xml"] => Command::Show { id: None, xml: true },
        ["show", id] => Command::Show { id: (*id != "last").then(|| (*id).into()), xml: false },
        ["show", id, "xml"] => Command::Show { id: (*id != "last").then(|| (*id).into()), xml: true },
        ["scenario", name] => Command::Scenario(name.parse()?),
        ["fault", "off"] => Command::Fault(None),
        ["fault", n] => match n.parse::<u8>() {
            Ok(n @ 1..=4) => Command::Fault(Some(n)),
            _ => return Err("fault takes 1, 2, 3, 4 or off".into()),
        },
        ["quit" | "exit" | "q"] => Command::Quit,
        _ => return Err(format!("unknown command {line:?}; type help")),
    })
}

pub fn execute(sandbox: &mut Sandbox, command: Command) -> Output {
    let mut out = Output::default();
    match command {
        Command::Help => out.lines = help().into_iter().chain(script::help()).collect(),
        Command::Quit => out.quit = true,
        Command::Pay { from, llave, amount } => match sandbox.pay(&from, &llave, amount) {
            Ok(id) => describe(sandbox, &id.to_string(), false, &mut out),
            Err(e) => out.push(Tone::Bad, e.to_string()),
        },
        Command::Resolve(llave) => match sandbox.resolve(&llave) {
            Ok(r) => {
                out.push(Tone::Heading, format!("resolve {llave}"));
                out.push(Tone::Plain, format!("  llave        {} (type {})", r.llave.value(), r.llave.kind()));
                out.push(Tone::Good, format!("  name         {}", r.name));
                out.push(Tone::Plain, format!("  institution  {} ({})", name_of(r.integrante), r.integrante));
                out.push(Tone::Plain, format!("  red          {}", r.red));
                out.push(Tone::Dim, "  Only the masked name leaves the beneficiary's red.");
            }
            Err(e) => out.push(Tone::Bad, e.to_string()),
        },
        Command::Llaves => {
            out.push(Tone::Heading, "Seeded llaves (fictional holders)");
            for l in &LLAVES {
                let masked = caudal_directory::MaskedName::mask(l.holder);
                out.push(
                    Tone::Plain,
                    format!("  {:<9} {:<22} {:<4} {:<14} {}", l.id, l.value, l.kind, l.institution, masked),
                );
            }
        }
        Command::Settle => {
            let s = sandbox.settle();
            out.push(Tone::Heading, format!("Malla settled at {}", clock(s.at)));
            if s.malla.payments.is_empty() {
                out.push(Tone::Dim, "  Nothing to settle: no payment posted since the last settlement.");
            }
            out.push(Tone::Plain, format!("  payments     {}", s.malla.payments.len()));
            out.push(Tone::Plain, format!("  gross        USD {}", money(i128::from(s.malla.gross().cents()))));
            out.push(Tone::Plain, format!("  net moved    USD {}", money(i128::from(s.malla.total().cents()))));
            for (b, net) in &s.malla.nets {
                let tone = if *net < 0 { Tone::Warn } else { Tone::Good };
                out.push(tone, format!("  {:<16} {:>14}", name_of(*b), money(*net)));
            }
            out.push(Tone::Dim, "  Positions closed against scheme:liquidado; committed headroom released.");
        }
        Command::Network => network(sandbox, &mut out),
        Command::Show { id, xml } => {
            let id = id.or_else(|| sandbox.last_payment().map(|r| r.id.to_string()));
            match id {
                Some(id) => describe(sandbox, &id, xml, &mut out),
                None => out.push(Tone::Bad, "no payment yet"),
            }
        }
        Command::Scenario(scenario) => {
            let run = scenario.run(None);
            out.lines.extend(script::render(&run.report));
            *sandbox = run.sandbox;
            out.push(Tone::Dim, "The panels now show the network at the end of the scenario.");
        }
        Command::Fault(n) => {
            let net = sandbox.network_mut();
            net.clear_faults();
            let sip = bic(crate::sandbox::SIP);
            let fault: Option<(caudal_south_api::Fault, &str)> = match n {
                None => None,
                Some(1) => Some((
                    Arc::new(move |_, t, m| t == sip && matches!(m, Message::Order(_))),
                    "the ordering red cannot reach the SIP",
                )),
                Some(2) => Some((
                    Arc::new(move |f, _, m| f == sip && matches!(m, Message::Order(_))),
                    "the SIP cannot deliver to the beneficiary red",
                )),
                Some(3) => Some((
                    Arc::new(move |_, t, m| {
                        t == sip && matches!(m, Message::Report(r) if r.state != PaymentState::Rcvd)
                    }),
                    "the beneficiary red cannot confirm to the SIP",
                )),
                Some(_) => Some((
                    Arc::new(move |f, _, m| {
                        f == sip && matches!(m, Message::Report(r) if r.state == PaymentState::Acsp)
                    }),
                    "the SIP cannot return the confirmation to the ordering red",
                )),
            };
            match fault {
                None => out.push(Tone::Good, "faults cleared"),
                Some((f, what)) => {
                    net.add_fault(f);
                    out.push(
                        Tone::Warn,
                        format!("exception {}: {what}. `fault off` clears it.", n.unwrap_or_default()),
                    );
                }
            }
        }
    }
    out
}

pub fn name_of(b: Bic) -> String {
    match institution_by_bic(b) {
        Some(i) => i.id.into(),
        None if b == bic(crate::sandbox::SIP) => "SIP".into(),
        None => crate::sandbox::REDES.iter().find(|r| r.bic == b.as_str()).map_or(b.to_string(), |r| r.id.into()),
    }
}

pub fn clock(at: jiff::Timestamp) -> String {
    to_scheme_time(at).strftime("%H:%M:%S%.3f").to_string()
}

pub fn state_label(row: &Row) -> String {
    let mut s = row.state.to_string();
    if let Some(r) = row.reason {
        s.push_str(&format!(" {r}"));
    }
    if row.por_conciliar {
        s.push_str(" por conciliar");
    }
    if row.in_process {
        s.push_str(" en proceso");
    }
    s
}

pub fn state_tone(row: &Row) -> Tone {
    match row.state {
        _ if row.por_conciliar || row.in_process => Tone::Warn,
        PaymentState::Acsp | PaymentState::Acsc => Tone::Good,
        PaymentState::Rjct => Tone::Bad,
        _ => Tone::Dim,
    }
}

fn hop_line(h: &Hop) -> String {
    let what = match (h.state, h.reason) {
        (Some(s), Some(r)) => format!("{s} {r}"),
        (Some(s), None) => s.to_string(),
        (None, _) => "order".into(),
    };
    let lost = if h.dropped { "  ✗ lost" } else { "" };
    format!("  {}  {:<14} → {:<14} {:<16} {what}{lost}", clock(h.at), name_of(h.from), name_of(h.to), h.definition)
}

fn describe(sandbox: &Sandbox, id: &str, xml: bool, out: &mut Output) {
    let Some((row, hops)) = sandbox.payment(id) else {
        out.push(Tone::Bad, format!("no payment {id:?}"));
        return;
    };
    out.push(Tone::Heading, format!("payment {}", row.id));
    out.push(Tone::Plain, format!("  {} → {}   USD {}", name_of(row.from), name_of(row.to), row.amount));
    out.push(Tone::Heading, "Hops");
    for h in &hops {
        out.push(if h.dropped { Tone::Bad } else { Tone::Plain }, hop_line(h));
        if xml {
            for l in caudal_iso20022::pretty(&h.xml).lines() {
                out.push(Tone::Xml, format!("      {l}"));
            }
        }
    }
    out.push(Tone::Heading, "Ledger");
    let entries = pollster::block_on(caudal_ledger::Ledger::entries(
        sandbox.network().ledger(),
        EntryQuery::Payment(row.id.clone()),
    ))
    .unwrap_or_default();
    for e in &entries {
        let kind = match &e.kind {
            caudal_ledger::EntryKind::Reserved(_) => "reserve (pending)",
            caudal_ledger::EntryKind::Posted(_) => "post (irrevocable)",
            caudal_ledger::EntryKind::Voided(..) => "void",
            caudal_ledger::EntryKind::Transferred(..) => "transfer",
        };
        out.push(Tone::Plain, format!("  {}  {kind}", clock(e.at)));
        for leg in &e.legs {
            let (d, c, units) = match leg {
                Leg::Obligations { debit, credit, amount } => {
                    (caudal_ledger::AccountId::from(*debit), caudal_ledger::AccountId::from(*credit), amount.cents())
                }
                Leg::Headroom { debit, credit, amount } => ((*debit).into(), (*credit).into(), amount.units()),
            };
            out.push(
                Tone::Dim,
                format!(
                    "      debit {:<32} credit {:<32} {:>12}",
                    d.to_string(),
                    c.to_string(),
                    money(i128::from(units))
                ),
            );
        }
    }
    if entries.is_empty() {
        out.push(Tone::Dim, "  none: the scheme ledger only books inter-red payments the SIP accepted");
    }
    out.push(state_tone(&row), format!("Result: {}", state_label(&row)));
    let note = match row.state {
        _ if row.por_conciliar => "Irrevocable, outcome unknown: it must be resolved today by 18:00.",
        _ if row.in_process => {
            "Irrevocable at the SIP; the ordering bank tells its customer the payment is in process."
        }
        PaymentState::Acsp => "Irrevocable and in today's malla. `settle` takes it to ACSC.",
        PaymentState::Acsc => "Settled: the malla closed and positions moved at the central bank.",
        PaymentState::Rjct => "Rejected: any reservation was voided and the headroom returned.",
        _ => "",
    };
    if !note.is_empty() {
        out.push(Tone::Dim, note);
    }
}

fn network(sandbox: &Sandbox, out: &mut Output) {
    out.push(Tone::Heading, format!("Network at {}", clock(sandbox.now())));
    out.push(
        Tone::Dim,
        format!(
            "  {:<14} {:<9} {:>6} {:>14} {:>14} {:>9} {:>12}",
            "institution", "red", "sector", "ceiling", "available", "light", "net"
        ),
    );
    for v in sandbox.institutions() {
        let tone = match v.light {
            caudal_risk::Light::Green => Tone::Good,
            caudal_risk::Light::Yellow | caudal_risk::Light::Orange => Tone::Warn,
            caudal_risk::Light::Red => Tone::Bad,
        };
        out.push(
            tone,
            format!(
                "  {:<14} {:<9} {:>6} {:>14} {:>14} {:>9} {:>12}",
                v.id,
                v.red.to_string(),
                v.sector.to_string(),
                money(i128::from(v.ceiling.cents())),
                money(v.available),
                v.light.label(),
                money(v.net)
            ),
        );
    }
}
