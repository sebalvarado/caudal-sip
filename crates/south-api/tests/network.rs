use std::sync::Arc;

use caudal_ledger::{AccountId, AsOf, Headroom, HeadroomAccount, Ledger, Leg, MemLedger};
use caudal_msgplane::SimClock;
use caudal_payments::{Answer, Deadlines, Integrante, Message, Order, Party, Red, Role, Script, Sip};
use caudal_registry::{Integrante as Member, Red as RedInfo, Registry, Sector, Status};
use caudal_south_api::Network;
use caudal_types::{Bic, PaymentId, PaymentState, ReasonCode, Usd};
use jiff::{SignedDuration, Timestamp};

fn bic(s: &str) -> Bic {
    s.parse().unwrap()
}

const SIP: &str = "SIPCECEQ";
const RED_A: &str = "RALFECEQ";
const RED_B: &str = "RBETECEQ";
const BANCO: &str = "BALFECEQ";
const COOP: &str = "CBETECEQ";
const FINTECH: &str = "FGAMECEQ";

fn start() -> Timestamp {
    "2026-10-12T14:00:00Z".parse().unwrap()
}

fn network(headroom: &str, script: Script) -> Network<MemLedger> {
    let mut registry = Registry::default();
    registry.add_red(RedInfo { bic: bic(RED_A), name: "red alfa".into() }).unwrap();
    registry.add_red(RedInfo { bic: bic(RED_B), name: "red beta".into() }).unwrap();
    for (b, red, sector) in
        [(BANCO, RED_A, Sector::Financial), (COOP, RED_B, Sector::CooperativeSegment3), (FINTECH, RED_B, Sector::Sedpe)]
    {
        let member = Member { bic: bic(b), name: b.into(), sector, red: bic(red), status: Status::Active };
        registry.add_integrante(member).unwrap();
    }
    let d = Deadlines::default();
    let roles = vec![
        Role::Sip(Sip::new(bic(SIP), registry, d)),
        Role::Red(Red::new(bic(RED_A), bic(SIP), [bic(BANCO)], d)),
        Role::Red(Red::new(bic(RED_B), bic(SIP), [bic(COOP), bic(FINTECH)], d)),
        Role::Integrante(Integrante::new(bic(BANCO), bic(RED_A), d, Script::accept())),
        Role::Integrante(Integrante::new(bic(COOP), bic(RED_B), d, script.clone())),
        Role::Integrante(Integrante::new(bic(FINTECH), bic(RED_B), d, script)),
    ];
    let clock = SimClock::starting_at(start());
    let ledger_clock = clock.clone();
    let ledger = MemLedger::new(move || ledger_clock.now());
    let mut net = Network::new(ledger, clock, roles, bic(SIP), b"test", SignedDuration::from_millis(5));
    let amount = Headroom::of(headroom.parse().unwrap());
    let legs = [BANCO, COOP, FINTECH]
        .into_iter()
        .map(|b| Leg::Headroom { debit: HeadroomAccount::Granted, credit: HeadroomAccount::Integrante(bic(b)), amount })
        .collect();
    net.fund(legs).unwrap();
    net
}

fn order(n: u32, from: &str, to: &str, amount: &str) -> Order {
    let red = |b: &str| bic(if b == BANCO { RED_A } else { RED_B });
    let party = |name: &str| Party { name: name.into(), document: "0000000000".into(), account: "1100000000".into() };
    Order {
        id: PaymentId::new(bic(from), &format!("{from}12102026090000{n:06}")).unwrap(),
        created: start(),
        ordering_red: red(from),
        beneficiary_red: red(to),
        beneficiary: bic(to),
        amount: amount.parse().unwrap(),
        debtor: party("ana ejemplo"),
        creditor: party("julio tienda"),
    }
}

fn pay(net: &mut Network<MemLedger>, o: Order) -> PaymentId {
    let id = o.id.clone();
    let now = net.now();
    net.schedule_payment(now, Order { created: now, ..o });
    net.run_until_idle();
    id
}

fn state(net: &Network<MemLedger>, id: &PaymentId) -> (PaymentState, Option<ReasonCode>) {
    let row = net.row(id).unwrap();
    (row.state, row.reason)
}

fn available(net: &Network<MemLedger>, b: &str) -> i128 {
    let account = AccountId::Headroom(HeadroomAccount::Integrante(bic(b)));
    pollster::block_on(net.ledger().balance(account, AsOf::Latest)).unwrap().available()
}

fn drop_when(f: impl Fn(Bic, Bic, &Message) -> bool + Send + Sync + 'static) -> caudal_south_api::Fault {
    Arc::new(f)
}

#[test]
fn a_payment_crosses_both_redes_reaches_acsp_then_acsc_on_settlement() {
    let mut net = network("1000.00", Script::accept());
    let id = pay(&mut net, order(1, BANCO, COOP, "25.40"));
    assert_eq!(state(&net, &id), (PaymentState::Acsp, None));
    let path: Vec<_> = net.hops(&id).iter().map(|h| (h.from.to_string(), h.to.to_string(), h.definition)).collect();
    assert!(path.contains(&(BANCO.into(), RED_A.into(), "pacs.008.001.08")));
    assert!(path.contains(&(RED_A.into(), SIP.into(), "pacs.008.001.08")));
    assert!(path.contains(&(SIP.into(), RED_B.into(), "pacs.008.001.08")));
    assert!(path.contains(&(RED_B.into(), COOP.into(), "pacs.008.001.08")));
    assert!(path.contains(&(SIP.into(), RED_A.into(), "pacs.002.001.10")));
    assert_eq!(available(&net, BANCO), 100_000 - 2_540);
    let s = net.settle().unwrap();
    net.run_until_idle();
    assert_eq!(s.malla.nets[&bic(BANCO)], -2_540);
    assert_eq!(s.malla.nets[&bic(COOP)], 2_540);
    assert_eq!(state(&net, &id), (PaymentState::Acsc, None));
    assert_eq!(available(&net, BANCO), 100_000, "settlement returns the headroom");
    assert_eq!(net.rejected_envelopes(), 0);
}

#[test]
fn exhausted_headroom_blocks_the_next_payment_with_am04() {
    let mut net = network("30.00", Script::accept());
    let first = pay(&mut net, order(1, COOP, BANCO, "25.00"));
    let second = pay(&mut net, order(2, COOP, BANCO, "10.00"));
    assert_eq!(state(&net, &first).0, PaymentState::Acsp);
    assert_eq!(state(&net, &second), (PaymentState::Rjct, Some(ReasonCode::Am04)));
    net.settle().unwrap();
    let third = pay(&mut net, order(3, COOP, BANCO, "10.00"));
    assert_eq!(state(&net, &third).0, PaymentState::Acsp, "a settled malla frees the headroom");
}

#[test]
fn a_beneficiary_rejection_reaches_the_ordering_bank_with_its_code() {
    let script =
        Script::new(
            |o: &Order| {
                if o.amount > Usd::from_cents(10_000) { Answer::Reject(ReasonCode::Ac06) } else { Answer::Accept }
            },
        );
    let mut net = network("1000.00", script);
    let id = pay(&mut net, order(1, BANCO, COOP, "500.00"));
    assert_eq!(state(&net, &id), (PaymentState::Rjct, Some(ReasonCode::Ac06)));
    assert_eq!(available(&net, BANCO), 100_000);
}

#[test]
fn exception_one_the_ordering_red_cannot_reach_the_sip() {
    let mut net = network("1000.00", Script::accept());
    net.add_fault(drop_when(|from, to, _| from == bic(RED_A) && to == bic(SIP)));
    let id = pay(&mut net, order(1, BANCO, COOP, "25.40"));
    assert_eq!(state(&net, &id), (PaymentState::Rjct, Some(ReasonCode::Ta01)));
    assert_eq!(available(&net, BANCO), 100_000, "nothing was reserved");
}

#[test]
fn exception_two_the_sip_cannot_deliver_to_the_beneficiary_red() {
    let mut net = network("1000.00", Script::accept());
    net.add_fault(drop_when(|from, to, _| from == bic(SIP) && to == bic(RED_B)));
    let id = pay(&mut net, order(1, BANCO, COOP, "25.40"));
    assert_eq!(state(&net, &id), (PaymentState::Rjct, Some(ReasonCode::Ta01)));
    assert_eq!(available(&net, BANCO), 100_000, "voided");
}

#[test]
fn exception_three_the_beneficiary_red_cannot_confirm() {
    let mut net = network("1000.00", Script::accept());
    net.add_fault(drop_when(|from, to, m| {
        from == bic(RED_B) && to == bic(SIP) && matches!(m, Message::Report(r) if r.state != PaymentState::Rcvd)
    }));
    let id = pay(&mut net, order(1, BANCO, COOP, "25.40"));
    let row = net.row(&id).unwrap();
    assert_eq!(row.state, PaymentState::Acsp);
    assert!(row.por_conciliar, "irrevocable, outcome unknown");
    net.settle().unwrap();
    net.run_until_idle();
    assert_eq!(state(&net, &id).0, PaymentState::Acsc);
}

#[test]
fn exception_four_the_ordering_red_never_hears_back() {
    let mut net = network("1000.00", Script::accept());
    net.add_fault(drop_when(|from, to, m| {
        from == bic(SIP) && to == bic(RED_A) && matches!(m, Message::Report(r) if r.state == PaymentState::Acsp)
    }));
    let id = pay(&mut net, order(1, BANCO, COOP, "25.40"));
    let row = net.row(&id).unwrap();
    assert_eq!(row.state, PaymentState::Acsp, "the SIP posted normally");
    assert!(row.in_process, "the ordering bank tells its customer the payment is in process");
    net.settle().unwrap();
    net.run_until_idle();
    let row = net.row(&id).unwrap();
    assert_eq!(row.state, PaymentState::Acsc);
    assert!(!row.in_process, "the settlement notice resolves it");
}

#[test]
fn an_intra_red_payment_stays_inside_the_red() {
    let mut net = network("1000.00", Script::accept());
    let id = pay(&mut net, order(1, COOP, FINTECH, "12.00"));
    assert_eq!(state(&net, &id).0, PaymentState::Acsp);
    assert!(net.hops(&id).iter().all(|h| h.from != bic(SIP) && h.to != bic(SIP)));
    assert_eq!(available(&net, COOP), 100_000, "the scheme ledger is not involved");
}

#[test]
fn a_silent_beneficiary_times_out_with_dt01_inside_twenty_seconds() {
    let mut net = network("1000.00", Script::new(|_| Answer::Silent));
    let id = pay(&mut net, order(1, BANCO, COOP, "25.40"));
    assert_eq!(state(&net, &id), (PaymentState::Rjct, Some(ReasonCode::Dt01)));
    let row = net.row(&id).unwrap();
    assert!(row.updated.duration_since(row.created) < SignedDuration::from_secs(20));
    assert_eq!(available(&net, BANCO), 100_000);
}

#[test]
fn the_same_inputs_replay_exactly() {
    let run = || {
        let mut net = network("200.00", Script::accept());
        for n in 0..40 {
            let (from, to) = [(BANCO, COOP), (COOP, BANCO), (FINTECH, BANCO), (BANCO, FINTECH)][n % 4];
            net.schedule_payment(
                start() + SignedDuration::from_millis(n as i64 * 7),
                order(n as u32, from, to, "9.99"),
            );
        }
        net.run_until_idle();
        net.settle().unwrap();
        net.run_until_idle();
        net.rows().to_vec()
    };
    assert_eq!(run(), run());
}
