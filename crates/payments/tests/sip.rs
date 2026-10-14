use caudal_ledger::testing::clock;
use caudal_ledger::{
    AccountId, AsOf, Headroom, HeadroomAccount, Ledger, LedgerError, Leg, MemLedger, ObligationAccount, Transfer,
    TransferId,
};
use caudal_payments::{
    Command, Ctx, Deadlines, Event, Input, Message, Order, Outcome, Output, Party, Report, Sip, Timer, TimerKind,
};
use caudal_registry::{Integrante, Red, Registry, Sector, Status};
use caudal_types::{Bic, PaymentId, PaymentState, ReasonCode, Usd};
use jiff::{SignedDuration, Timestamp};
use pollster::block_on;
use uuid::Uuid;

fn bic(s: &str) -> Bic {
    s.parse().unwrap()
}

const RED_A: &str = "RALFECEQ";
const RED_B: &str = "RBETECEQ";

fn registry() -> Registry {
    let mut r = Registry::default();
    for (red, name) in [(RED_A, "red alfa"), (RED_B, "red beta")] {
        r.add_red(Red { bic: bic(red), name: name.into() }).unwrap();
    }
    for (b, red) in [("BALFECEQ", RED_A), ("CBETECEQ", RED_B)] {
        let i = Integrante {
            bic: bic(b),
            name: b.into(),
            sector: Sector::Financial,
            red: bic(red),
            status: Status::Active,
        };
        r.add_integrante(i).unwrap();
    }
    r
}

fn start() -> Timestamp {
    "2026-10-12T14:00:00Z".parse().unwrap()
}

fn order(n: u32, cents: u64) -> Order {
    let party = |name: &str| Party { name: name.into(), document: "0000000000".into(), account: "0000000000".into() };
    Order {
        id: PaymentId::new(bic("BALFECEQ"), &format!("BALFECEQ12102026090000{n:06}")).unwrap(),
        created: start(),
        ordering_red: bic(RED_A),
        beneficiary_red: bic(RED_B),
        beneficiary: bic("CBETECEQ"),
        amount: Usd::from_cents(cents),
        debtor: party("ana"),
        creditor: party("julio"),
    }
}

struct Harness {
    sip: Sip,
    ledger: MemLedger,
    now: Timestamp,
    n: u128,
    sent: Vec<(Bic, Message)>,
    events: Vec<Event>,
    timers: Vec<(Timestamp, Timer)>,
}

impl Harness {
    fn new(headroom_cents: u64) -> Self {
        let ledger = MemLedger::new(clock());
        let legs = vec![Leg::Headroom {
            debit: HeadroomAccount::Granted,
            credit: HeadroomAccount::Integrante(bic("BALFECEQ")),
            amount: Headroom::of(Usd::from_cents(headroom_cents)),
        }];
        let cause = caudal_ledger::Cause { event: Uuid::nil(), digest: [0; 32] };
        block_on(ledger.transfer(Transfer {
            id: TransferId(Uuid::from_u128(1)),
            purpose: caudal_ledger::Purpose::Headroom,
            legs,
            cause,
        }))
        .unwrap();
        let sip = Sip::new(bic("SIPCECEQ"), registry(), Deadlines::default());
        Self { sip, ledger, now: start(), n: 100, sent: vec![], events: vec![], timers: vec![] }
    }

    fn step(&mut self, input: Input) {
        self.n += 1;
        let ctx = Ctx { now: self.now, event: Uuid::from_u128(self.n), digest: [1; 32] };
        for output in self.sip.step(&ctx, input) {
            match output {
                Output::Send { to, message } => self.sent.push((to, message)),
                Output::Event(e) => self.events.push(e),
                Output::Arm { at, timer } => self.timers.push((at, timer)),
                Output::Ledger(command) => {
                    let result = match command.clone() {
                        Command::Reserve(r) => block_on(self.ledger.reserve(r)).map(|a| a.effect),
                        Command::Post(p) => block_on(self.ledger.post(p)).map(|a| a.effect),
                        Command::Void(p, reason) => block_on(self.ledger.void(p, reason)).map(|a| a.effect),
                        Command::Transfer(t) => block_on(self.ledger.transfer(t)).map(|a| a.effect),
                    };
                    let result = result.map_err(|e| match e {
                        LedgerError::Refused(r) => r,
                        LedgerError::Unknown(e) => panic!("{e}"),
                    });
                    self.step(Input::Ledger(Outcome { command, result }));
                }
            }
        }
    }

    fn receive(&mut self, from: &str, message: Message) {
        self.step(Input::Received { from: bic(from), message });
    }

    fn fire(&mut self, kind: TimerKind) {
        let (at, timer) = self.timers.iter().find(|(_, t)| t.kind == kind).cloned().expect("armed");
        self.now = at;
        self.step(Input::Deadline(timer));
    }

    fn reports_to(&self, to: &str) -> Vec<(PaymentState, Option<ReasonCode>)> {
        self.sent
            .iter()
            .filter(|(b, _)| *b == bic(to))
            .filter_map(|(_, m)| match m {
                Message::Report(r) => Some((r.state, r.reason)),
                Message::Order(_) => None,
            })
            .collect()
    }

    fn available(&self) -> i128 {
        let account = AccountId::Headroom(HeadroomAccount::Integrante(bic("BALFECEQ")));
        block_on(self.ledger.balance(account, AsOf::Latest)).unwrap().available()
    }
}

fn ben_report(o: &Order, state: PaymentState, reason: Option<ReasonCode>) -> Message {
    Message::Report(Report::new(&o.id, state, reason))
}

#[test]
fn happy_path_reserves_delivers_posts_and_settles() {
    let mut h = Harness::new(100_000);
    let o = order(1, 2_540);
    h.receive(RED_A, Message::Order(o.clone()));
    assert_eq!(h.available(), 100_000 - 2_540);
    assert!(matches!(&h.sent[1], (to, Message::Order(_)) if *to == bic(RED_B)));
    h.receive(RED_B, ben_report(&o, PaymentState::Rcvd, None));
    h.receive(RED_B, ben_report(&o, PaymentState::Acsp, None));
    assert_eq!(h.reports_to(RED_A), [(PaymentState::Rcvd, None), (PaymentState::Acsp, None)]);
    h.step(Input::Settled(vec![o.id.clone()]));
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Acsc, None)));
    assert_eq!(h.sip.state(&o.id), Some((PaymentState::Acsc, None)));
}

#[test]
fn exhausted_headroom_rejects_with_am04_before_the_beneficiary_red() {
    let mut h = Harness::new(1_000);
    h.receive(RED_A, Message::Order(order(1, 2_540)));
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Am04))));
    assert!(h.sent.iter().all(|(to, _)| *to != bic(RED_B)));
}

#[test]
fn beneficiary_rejection_voids_and_returns_headroom() {
    let mut h = Harness::new(100_000);
    let o = order(1, 2_540);
    h.receive(RED_A, Message::Order(o.clone()));
    h.receive(RED_B, ben_report(&o, PaymentState::Rjct, Some(ReasonCode::Ac01)));
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Ac01))));
    assert_eq!(h.available(), 100_000);
}

#[test]
fn exception_two_undelivered_voids_with_ta01() {
    let mut h = Harness::new(100_000);
    h.receive(RED_A, Message::Order(order(1, 2_540)));
    h.fire(TimerKind::Deliver);
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Ta01))));
    assert_eq!(h.available(), 100_000);
}

#[test]
fn exception_three_unconfirmed_posts_and_goes_por_conciliar() {
    let mut h = Harness::new(100_000);
    let o = order(1, 2_540);
    h.receive(RED_A, Message::Order(o.clone()));
    h.receive(RED_B, ben_report(&o, PaymentState::Rcvd, None));
    h.fire(TimerKind::Confirm);
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Acsp, None)));
    assert!(h.events.contains(&Event::PorConciliar(o.id.clone())));
    let suspense = AccountId::Obligations(ObligationAccount::Suspense);
    assert_eq!(block_on(h.ledger.balance(suspense, AsOf::Latest)).unwrap().credits_posted, 2_540);
    h.receive(RED_B, ben_report(&o, PaymentState::Rjct, Some(ReasonCode::Ac01)));
    assert_eq!(h.available(), 100_000 - 2_540, "irrevocable: a late rejection changes nothing");
}

#[test]
fn a_repeated_order_is_answered_again_and_a_conflicting_one_is_du01() {
    let mut h = Harness::new(100_000);
    let o = order(1, 2_540);
    h.receive(RED_A, Message::Order(o.clone()));
    h.receive(RED_B, ben_report(&o, PaymentState::Acsp, None));
    h.receive(RED_A, Message::Order(o.clone()));
    assert_eq!(h.reports_to(RED_A)[2..], [(PaymentState::Rcvd, None), (PaymentState::Acsp, None)]);
    assert_eq!(h.available(), 100_000 - 2_540, "reserved once");
    h.receive(RED_A, Message::Order(Order { amount: Usd::from_cents(1), ..o }));
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Du01))));
}

#[test]
fn validation_refuses_before_reserving() {
    let mut h = Harness::new(100_000);
    h.receive(RED_A, Message::Order(order(1, 0)));
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Am01))));
    h.now = start() + SignedDuration::from_secs(21);
    h.receive(RED_A, Message::Order(order(2, 100)));
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Dt01))));
    h.receive(RED_B, Message::Order(order(3, 100)));
    assert_eq!(h.reports_to(RED_B).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Ms01))));
    h.sip.registry_mut().set_status(bic("BALFECEQ"), Status::Suspended).unwrap();
    h.now = start();
    h.receive(RED_A, Message::Order(order(4, 100)));
    assert_eq!(h.reports_to(RED_A).last(), Some(&(PaymentState::Rjct, Some(ReasonCode::Am04))));
    assert_eq!(h.available(), 100_000);
}
