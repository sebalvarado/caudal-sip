use caudal_ledger::testing::{bic, cause, clock, payment};
use caudal_ledger::{
    AccountId, AsOf, Balance, Effect, EntryId, EntryKind, EntryQuery, Headroom, HeadroomAccount as H, Ledger,
    LedgerError, Leg, MemLedger, ObligationAccount as O, Purpose, Refusal, ReservationState, Reserve, Transfer,
    TransferId,
};
use caudal_types::{ReasonCode, Usd};
use pollster::block_on;
use uuid::Uuid;

fn usd(s: &str) -> Usd {
    s.parse().unwrap()
}

fn funded(amount: &str) -> MemLedger {
    let ledger = MemLedger::new(clock());
    let legs =
        vec![Leg::Headroom { debit: H::Granted, credit: H::Integrante(bic(0)), amount: Headroom::of(usd(amount)) }];
    block_on(ledger.transfer(Transfer {
        id: TransferId(Uuid::from_u128(1)),
        purpose: Purpose::Headroom,
        legs,
        cause: cause(1),
    }))
    .unwrap();
    ledger
}

fn reserve(n: u8) -> Reserve {
    Reserve {
        payment: payment(0, n),
        beneficiary: bic(1),
        amount: usd("100.00"),
        fee: usd("0.25"),
        tax: usd("0.04"),
        cause: cause(n),
    }
}

fn balance(ledger: &MemLedger, account: impl Into<AccountId>) -> Balance {
    block_on(ledger.balance(account.into(), AsOf::Latest)).unwrap()
}

fn refused<T: std::fmt::Debug>(r: Result<T, LedgerError>) -> Refusal {
    match r {
        Err(LedgerError::Refused(refusal)) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    }
}

#[test]
fn every_beneficiary_rejection_voids_both_books() {
    for (n, reason) in [ReasonCode::Ac04, ReasonCode::Ac01, ReasonCode::Ac06, ReasonCode::Ac05, ReasonCode::Ac07]
        .into_iter()
        .enumerate()
    {
        let ledger = funded("1000.00");
        block_on(ledger.reserve(reserve(n as u8))).unwrap();
        let r = block_on(ledger.void(payment(0, n as u8), reason)).unwrap();
        assert_eq!(r.value.state, ReservationState::Voided(reason));
        assert_eq!(balance(&ledger, H::Integrante(bic(0))).available(), 100_000);
        assert_eq!(balance(&ledger, H::Committed), Balance::default());
        assert_eq!(balance(&ledger, O::Position(bic(0))), Balance::default());
    }
}

#[test]
fn a_reserve_refused_for_headroom_leaves_nothing_behind() {
    let ledger = funded("50.00");
    assert_eq!(refused(block_on(ledger.reserve(reserve(1)))), Refusal::InsufficientHeadroom);
    assert_eq!(block_on(ledger.entries(EntryQuery::All)).unwrap().len(), 1);
    assert_eq!(refused(block_on(ledger.void(payment(0, 1), ReasonCode::Dt01))), Refusal::NotReserved);
}

#[test]
fn exceptions_one_and_two_void_and_cannot_post_afterwards() {
    let ledger = funded("1000.00");
    block_on(ledger.reserve(reserve(1))).unwrap();
    block_on(ledger.void(payment(0, 1), ReasonCode::Ta01)).unwrap();
    assert_eq!(refused(block_on(ledger.post(payment(0, 1)))), Refusal::Voided);
}

#[test]
fn exception_three_posts_then_parks_a_zero_net_memo_in_suspense() {
    let ledger = funded("1000.00");
    block_on(ledger.reserve(reserve(1))).unwrap();
    block_on(ledger.post(payment(0, 1))).unwrap();
    let memo = Transfer {
        id: TransferId(Uuid::from_u128(30)),
        purpose: Purpose::Suspense,
        legs: vec![Leg::Obligations { debit: O::Suspense, credit: O::Suspense, amount: usd("100.00") }],
        cause: cause(3),
    };
    block_on(ledger.transfer(memo)).unwrap();
    let suspense = balance(&ledger, O::Suspense);
    assert_eq!((suspense.debits_posted, suspense.credits_posted), (10_000, 10_000));
    assert_eq!(balance(&ledger, O::Position(bic(1))).credits_posted, 10_000, "the obligation stands");
    assert_eq!(refused(block_on(ledger.void(payment(0, 1), ReasonCode::Dt01))), Refusal::Irrevocable);
}

#[test]
fn a_reversal_mirrors_every_leg_and_a_repeat_appends_nothing() {
    let ledger = funded("1000.00");
    let grant = TransferId(Uuid::from_u128(1));
    let reversed = block_on(ledger.reverse(grant, cause(4))).unwrap();
    assert_eq!(reversed.effect, Effect::Appended);
    assert_eq!(reversed.value, EntryId::Reversal(grant));
    assert_eq!(balance(&ledger, H::Integrante(bic(0))).available(), 0, "the grant is undone, headroom included");
    let again = block_on(ledger.reverse(grant, cause(5))).unwrap();
    assert_eq!(again.effect, Effect::Replay);
    assert_eq!(again.value, reversed.value);
    let log = block_on(ledger.entries(EntryQuery::All)).unwrap();
    assert!(matches!(log[0].kind, EntryKind::Transferred { .. }), "the original entry is still there");
    assert_eq!(log.len(), 2);
    assert_eq!(refused(block_on(ledger.reverse(TransferId(Uuid::from_u128(99)), cause(6)))), Refusal::NotTransferred);
}

#[test]
fn a_reversal_that_would_overdraw_headroom_appends_nothing() {
    let ledger = funded("1000.00");
    block_on(ledger.reserve(reserve(1))).unwrap();
    assert_eq!(
        refused(block_on(ledger.reverse(TransferId(Uuid::from_u128(1)), cause(4)))),
        Refusal::InsufficientHeadroom
    );
    assert_eq!(block_on(ledger.entries(EntryQuery::All)).unwrap().len(), 2, "the grant and the reservation stand");
    block_on(ledger.void(payment(0, 1), ReasonCode::Dt01)).unwrap();
    assert_eq!(balance(&ledger, H::Integrante(bic(0))).available(), 100_000, "the reservation still releases");
}
