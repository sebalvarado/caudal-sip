use std::sync::atomic::{AtomicI64, Ordering};

use caudal_types::{Bic, PaymentId, ReasonCode, Usd};
use jiff::Timestamp;
use proptest::prelude::*;
use uuid::Uuid;

use crate::core::Replay;
use crate::{
    AccountId, AsOf, Book, Cause, Entry, EntryId, EntryQuery, Headroom, HeadroomAccount, Ledger, LedgerError, Leg,
    ObligationAccount, Position, Refusal, Reservation, Reserve, Transfer, TransferId,
};

// Fictional: no real institution appears in fixtures.
pub const BICS: [&str; 3] = ["TESTAAEC", "TESTBBEC", "TESTCCEC"];

pub fn bic(i: usize) -> Bic {
    BICS[i % BICS.len()].parse().expect("fixture BICs are valid")
}

pub fn payment(i: usize, n: u8) -> PaymentId {
    let ordering = bic(i);
    PaymentId::new(ordering, &format!("{ordering}12102026090000{n:06}")).expect("fixture ids are valid")
}

pub fn cause(seed: u8) -> Cause {
    Cause { event: Uuid::from_u128(u128::from(seed)), digest: [seed; 32] }
}

pub fn clock() -> impl Fn() -> Timestamp + Send + Sync + 'static {
    let start = "2026-10-12T14:00:00Z".parse::<Timestamp>().expect("valid").as_millisecond();
    let tick = AtomicI64::new(0);
    move || Timestamp::from_millisecond(start + tick.fetch_add(1, Ordering::Relaxed)).expect("in range")
}

#[derive(Debug, Clone)]
pub enum Op {
    Reserve(Reserve),
    Post(PaymentId),
    Void(PaymentId, ReasonCode),
    Transfer(Transfer),
}

fn obligation_account() -> impl Strategy<Value = ObligationAccount> {
    use ObligationAccount::*;
    prop_oneof![
        3 => (0..BICS.len()).prop_map(|i| Position(bic(i))),
        1 => Just(FeesReceivable),
        1 => Just(TaxesPayable),
        1 => Just(Settled),
        1 => Just(Suspense),
    ]
}

fn headroom_account() -> impl Strategy<Value = HeadroomAccount> {
    use HeadroomAccount::*;
    prop_oneof![3 => (0..BICS.len()).prop_map(|i| Integrante(bic(i))), 2 => Just(Granted), 1 => Just(Committed)]
}

fn leg() -> impl Strategy<Value = Leg> {
    prop_oneof![
        (obligation_account(), obligation_account(), 0..30_000u64).prop_map(|(debit, credit, c)| Leg::Obligations {
            debit,
            credit,
            amount: Usd::from_cents(c)
        }),
        (headroom_account(), headroom_account(), 0..30_000u64).prop_map(|(debit, credit, u)| Leg::Headroom {
            debit,
            credit,
            amount: Headroom::of(Usd::from_cents(u))
        }),
    ]
}

pub fn op() -> impl Strategy<Value = Op> {
    let cents = |max: u64| (0..max).prop_map(Usd::from_cents);
    let reserve = (0..BICS.len(), 0..BICS.len(), 0..4u8, cents(20_000), cents(100), cents(20), any::<u8>()).prop_map(
        |(i, j, n, amount, fee, tax, seed)| {
            Op::Reserve(Reserve { payment: payment(i, n), beneficiary: bic(j), amount, fee, tax, cause: cause(seed) })
        },
    );
    let fund = (0..BICS.len(), 0..50_000u64, any::<u8>()).prop_map(|(i, u, id)| {
        let legs = vec![Leg::Headroom {
            debit: HeadroomAccount::Granted,
            credit: HeadroomAccount::Integrante(bic(i)),
            amount: Headroom::of(Usd::from_cents(u)),
        }];
        Op::Transfer(Transfer { id: TransferId(Uuid::from_u128(u128::from(id))), legs, cause: cause(id) })
    });
    let transfer = (0..8u8, prop::collection::vec(leg(), 1..4)).prop_map(|(id, legs)| {
        Op::Transfer(Transfer { id: TransferId(Uuid::from_u128(1000 + u128::from(id))), legs, cause: cause(id) })
    });
    prop_oneof![
        4 => reserve,
        3 => (0..BICS.len(), 0..4u8).prop_map(|(i, n)| Op::Post(payment(i, n))),
        2 => (0..BICS.len(), 0..4u8, 0..ReasonCode::ALL.len()).prop_map(|(i, n, r)| Op::Void(payment(i, n), ReasonCode::ALL[r])),
        2 => fund,
        2 => transfer,
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Answer {
    Reservation(Reservation),
    Entry(EntryId),
    Refused(Refusal),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trace {
    pub answers: Vec<Answer>,
    pub entries: Vec<Entry>,
    pub positions: Vec<Position>,
}

async fn execute(ledger: &impl Ledger, op: &Op) -> Answer {
    let answer = match op.clone() {
        Op::Reserve(r) => ledger.reserve(r).await.map(Answer::Reservation),
        Op::Post(p) => ledger.post(p).await.map(Answer::Reservation),
        Op::Void(p, reason) => ledger.void(p, reason).await.map(Answer::Reservation),
        Op::Transfer(t) => ledger.transfer(t).await.map(Answer::Entry),
    };
    match answer {
        Ok(a) => a,
        Err(LedgerError::Refused(r)) => Answer::Refused(r),
        Err(LedgerError::Unknown(e)) => panic!("outcome unknown: {e}"),
    }
}

async fn positions(ledger: &impl Ledger, at: AsOf) -> Vec<Position> {
    let mut all = ledger.positions(Book::Obligations, at).await.expect("readable");
    all.extend(ledger.positions(Book::Headroom, at).await.expect("readable"));
    all
}

pub async fn run(ledger: &impl Ledger, ops: &[Op]) -> Trace {
    let mut answers = Vec::new();
    let mut log: Vec<Entry> = Vec::new();
    let mut snapshots = Vec::new();
    for op in ops {
        let answer = execute(ledger, op).await;
        let entries = ledger.entries(EntryQuery::All).await.expect("readable");
        assert_eq!(execute(ledger, op).await, answer, "a repeat answers the same: {op:?}");
        assert_eq!(ledger.entries(EntryQuery::All).await.expect("readable"), entries, "a repeat appends nothing");
        assert!(entries.starts_with(&log), "postings are immutable");
        assert!(entries.len() <= log.len() + 1, "one command appends at most one entry");
        let now = positions(ledger, AsOf::Latest).await;
        check(&now, &entries);
        if let Some(last) = entries.last().filter(|_| entries.len() > log.len()) {
            snapshots.push((last.at, now.clone()));
        }
        answers.push(answer);
        log = entries;
    }
    assert!(log.windows(2).all(|w| w[0].at < w[1].at), "commit times strictly increase");
    for (at, expected) in snapshots {
        assert_eq!(positions(ledger, AsOf::At(at)).await, expected, "a past read sees exactly what was committed");
    }
    if let Some(p) = ops.iter().find_map(|op| if let Op::Reserve(r) = op { Some(&r.payment) } else { None }) {
        let mine = ledger.entries(EntryQuery::Payment(p.clone())).await.expect("readable");
        assert!(mine.iter().all(|e| e.payment() == Some(p)), "a payment query returns only its entries");
    }
    Trace { answers, entries: log, positions: positions(ledger, AsOf::Latest).await }
}

fn check(positions: &[Position], entries: &[Entry]) {
    assert_eq!(positions, Replay::of(entries).positions(), "balances are derived from the log");
    for book in [Book::Obligations, Book::Headroom] {
        let sum = |f: fn(&Position) -> u128| positions.iter().filter(|p| p.account.book() == book).map(f).sum::<u128>();
        assert_eq!(
            sum(|p| p.balance.debits_pending),
            sum(|p| p.balance.credits_pending),
            "{book:?} pending sums to zero"
        );
        assert_eq!(sum(|p| p.balance.debits_posted), sum(|p| p.balance.credits_posted), "{book:?} posted sums to zero");
    }
    for p in positions.iter().filter(|p| p.account.no_overdraft()) {
        assert!(p.balance.available() >= 0, "{} is overdrawn", p.account);
    }
    let pending_obligations: u128 = positions
        .iter()
        .filter(|p| matches!(p.account, AccountId::Obligations(ObligationAccount::Position(_))))
        .map(|p| p.balance.debits_pending)
        .sum();
    let committed = positions
        .iter()
        .find(|p| p.account == HeadroomAccount::Committed.into())
        .map_or(0, |p| p.balance.credits_pending);
    assert_eq!(pending_obligations, committed, "a reservation is atomic across both books");
}
