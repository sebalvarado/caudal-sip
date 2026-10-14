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

fn grant(ledger: &MemLedger, i: usize, amount: &str) {
    let amount = usd(amount);
    let id = TransferId(Uuid::from_u128(((i as u128) << 64) | u128::from(amount.cents())));
    let legs = vec![Leg::Headroom { debit: H::Granted, credit: H::Integrante(bic(i)), amount: Headroom::of(amount) }];
    block_on(ledger.transfer(Transfer { id, purpose: Purpose::Headroom, legs, cause: cause(0) })).unwrap();
}

fn reserve(n: u8, amount: &str) -> Reserve {
    Reserve {
        payment: payment(0, n),
        beneficiary: bic(1),
        amount: usd(amount),
        fee: usd("0.25"),
        tax: usd("0.04"),
        cause: cause(n),
    }
}

fn balance(ledger: &MemLedger, account: impl Into<AccountId>) -> Balance {
    block_on(ledger.balance(account.into(), AsOf::Latest)).unwrap()
}

fn refusal<T: std::fmt::Debug>(r: Result<T, LedgerError>) -> Refusal {
    match r {
        Err(LedgerError::Refused(refusal)) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    }
}

#[test]
fn a_reservation_holds_headroom_and_creates_pending_obligations() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "1000.00");
    let r = block_on(ledger.reserve(reserve(1, "100.00"))).unwrap();
    assert_eq!(r.effect, Effect::Appended);
    assert_eq!(r.value.state, ReservationState::Pending);

    let pending = |debit, credit| Balance { debits_pending: debit, credits_pending: credit, ..Balance::default() };
    assert_eq!(
        balance(&ledger, H::Integrante(bic(0))),
        Balance { debits_pending: 10_029, credits_posted: 100_000, ..Balance::default() }
    );
    assert_eq!(balance(&ledger, H::Committed), pending(0, 10_029));
    assert_eq!(balance(&ledger, O::Position(bic(0))), pending(10_029, 0));
    assert_eq!(balance(&ledger, O::Position(bic(1))), pending(0, 10_000));
    assert_eq!(balance(&ledger, O::FeesReceivable), pending(0, 25));
    assert_eq!(balance(&ledger, O::TaxesPayable), pending(0, 4));

    block_on(ledger.post(payment(0, 1))).unwrap();
    assert_eq!(balance(&ledger, O::Position(bic(1))), Balance { credits_posted: 10_000, ..Balance::default() });
    assert_eq!(block_on(ledger.entries(EntryQuery::Payment(payment(0, 1)))).unwrap().len(), 2);
}

#[test]
fn a_void_releases_everything() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "1000.00");
    block_on(ledger.reserve(reserve(1, "100.00"))).unwrap();
    let r = block_on(ledger.void(payment(0, 1), ReasonCode::Ac04)).unwrap();
    assert_eq!(r.value.state, ReservationState::Voided(ReasonCode::Ac04));
    assert_eq!(balance(&ledger, H::Integrante(bic(0))).available(), 100_000);
    assert_eq!(balance(&ledger, O::Position(bic(1))), Balance::default());
}

#[test]
fn refuses_what_headroom_cannot_carry() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "100.28");
    assert_eq!(refusal(block_on(ledger.reserve(reserve(1, "100.00")))), Refusal::InsufficientHeadroom);
    grant(&ledger, 0, "0.01");
    block_on(ledger.reserve(reserve(1, "100.00"))).unwrap();
    assert_eq!(balance(&ledger, H::Integrante(bic(0))).available(), 0);
}

#[test]
fn a_repeat_returns_the_original_and_a_conflict_is_refused() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "1000.00");
    let first = block_on(ledger.reserve(reserve(1, "100.00"))).unwrap();
    let requoted = Reserve { fee: usd("0.30"), ..reserve(1, "100.00") };
    let again = block_on(ledger.reserve(requoted)).unwrap();
    assert_eq!(again.effect, Effect::Replay);
    assert_eq!(again.value, first.value);
    assert_eq!(refusal(block_on(ledger.reserve(reserve(1, "100.01")))), Refusal::Conflict);
    let elsewhere = Reserve { beneficiary: bic(2), ..reserve(1, "100.00") };
    assert_eq!(refusal(block_on(ledger.reserve(elsewhere))), Refusal::Conflict);
}

#[test]
fn post_and_void_follow_the_reservation() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "1000.00");
    assert_eq!(refusal(block_on(ledger.post(payment(0, 1)))), Refusal::NotReserved);
    block_on(ledger.reserve(reserve(1, "100.00"))).unwrap();
    block_on(ledger.reserve(reserve(2, "100.00"))).unwrap();
    block_on(ledger.post(payment(0, 1))).unwrap();
    block_on(ledger.void(payment(0, 2), ReasonCode::Dt01)).unwrap();
    assert_eq!(refusal(block_on(ledger.void(payment(0, 1), ReasonCode::Dt01))), Refusal::Irrevocable);
    assert_eq!(refusal(block_on(ledger.post(payment(0, 2)))), Refusal::Voided);
    let again = block_on(ledger.void(payment(0, 2), ReasonCode::Ac01)).unwrap();
    assert_eq!(again.effect, Effect::Replay);
    assert_eq!(again.value.state, ReservationState::Voided(ReasonCode::Dt01), "a repeat keeps the original reason");
}

#[test]
fn a_transfer_id_with_different_legs_is_a_conflict() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "10.00");
    let first = block_on(ledger.entries(EntryQuery::All)).unwrap().remove(0);
    let EntryKind::Transferred { id, purpose, cause } = first.kind else { unreachable!() };
    let legs =
        vec![Leg::Headroom { debit: H::Granted, credit: H::Integrante(bic(0)), amount: Headroom::of(usd("5.00")) }];
    assert_eq!(refusal(block_on(ledger.transfer(Transfer { id, purpose, legs, cause }))), Refusal::Conflict);
    let same = Transfer { id, purpose, legs: first.legs.clone(), cause };
    assert_eq!(block_on(ledger.transfer(same)).unwrap().value, EntryId::Transfer(id));
    let other = Transfer { id, purpose: Purpose::Correction, legs: first.legs, cause };
    assert_eq!(refusal(block_on(ledger.transfer(other))), Refusal::Conflict);
}

#[test]
fn a_transfer_cannot_overdraw_headroom() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "10.00");
    let withdraw = |id, amount| Transfer {
        id: TransferId(Uuid::from_u128(id)),
        legs: vec![Leg::Headroom {
            debit: H::Integrante(bic(0)),
            credit: H::Granted,
            amount: Headroom::of(usd(amount)),
        }],
        purpose: Purpose::Headroom,
        cause: cause(0),
    };
    assert_eq!(refusal(block_on(ledger.transfer(withdraw(10, "10.01")))), Refusal::InsufficientHeadroom);
    block_on(ledger.transfer(withdraw(11, "10.00"))).unwrap();
}

#[test]
fn reads_at_a_past_commit_time() {
    let ledger = MemLedger::new(clock());
    grant(&ledger, 0, "10.00");
    let at = block_on(ledger.entries(EntryQuery::All)).unwrap()[0].at;
    grant(&ledger, 1, "99.00");
    grant(&ledger, 0, "5.00");
    let headroom = |as_of| block_on(ledger.balance(H::Integrante(bic(0)).into(), as_of)).unwrap().credits_posted;
    assert_eq!(headroom(AsOf::At(at)), 1_000);
    assert_eq!(headroom(AsOf::Latest), 1_500);
}
