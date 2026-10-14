use std::collections::HashMap;

use caudal_types::{PaymentId, ReasonCode};
use jiff::{SignedDuration, Timestamp};

use crate::{
    AccountId, Balance, Draft, Entry, EntryId, EntryKind, Headroom, HeadroomAccount, Leg, ObligationAccount, Refusal,
    Reservation, ReservationState, Reserve, Transfer, TransferId,
};

pub trait View {
    fn reservation(&self, payment: &PaymentId) -> Option<Reservation>;
    fn transfer(&self, id: TransferId) -> Option<Vec<Leg>>;
    fn balance(&self, account: AccountId) -> Balance;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision<T> {
    pub answer: T,
    pub draft: Option<Draft>,
}

impl<T> Decision<T> {
    fn repeat(answer: T) -> Result<Self, Refusal> {
        Ok(Self { answer, draft: None })
    }

    fn commit(view: &impl View, answer: T, draft: Draft) -> Result<Self, Refusal> {
        refuse_overdraft(view, &draft)?;
        Ok(Self { answer, draft: Some(draft) })
    }
}

pub fn reserve(view: &impl View, r: Reserve) -> Result<Decision<Reservation>, Refusal> {
    if let Some(existing) = view.reservation(&r.payment) {
        if existing.amount != r.amount || existing.beneficiary != r.beneficiary {
            return Err(Refusal::Conflict);
        }
        return Decision::repeat(existing);
    }
    r.total().ok_or(Refusal::InsufficientHeadroom)?;
    let reservation = Reservation::pending(&r);
    let legs = reservation_legs(&reservation);
    Decision::commit(view, reservation, Draft { kind: EntryKind::Reserved(r), legs })
}

pub fn post(view: &impl View, payment: &PaymentId) -> Result<Decision<Reservation>, Refusal> {
    let mut r = view.reservation(payment).ok_or(Refusal::NotReserved)?;
    match r.state {
        ReservationState::Posted => Decision::repeat(r),
        ReservationState::Voided(_) => Err(Refusal::Voided),
        ReservationState::Pending => {
            r.state = ReservationState::Posted;
            let legs = reservation_legs(&r);
            Decision::commit(view, r, Draft { kind: EntryKind::Posted(payment.clone()), legs })
        }
    }
}

pub fn void(view: &impl View, payment: &PaymentId, reason: ReasonCode) -> Result<Decision<Reservation>, Refusal> {
    let mut r = view.reservation(payment).ok_or(Refusal::NotReserved)?;
    match r.state {
        ReservationState::Voided(_) => Decision::repeat(r),
        ReservationState::Posted => Err(Refusal::Irrevocable),
        ReservationState::Pending => {
            r.state = ReservationState::Voided(reason);
            let legs = reservation_legs(&r);
            Decision::commit(view, r, Draft { kind: EntryKind::Voided(payment.clone(), reason), legs })
        }
    }
}

pub fn transfer(view: &impl View, t: Transfer) -> Result<Decision<EntryId>, Refusal> {
    let id = EntryId::Transfer(t.id);
    match view.transfer(t.id) {
        Some(legs) if legs == t.legs => Decision::repeat(id),
        Some(_) => Err(Refusal::Conflict),
        None => Decision::commit(view, id, Draft { kind: EntryKind::Transferred(t.id, t.cause), legs: t.legs }),
    }
}

pub fn reservation_legs(r: &Reservation) -> Vec<Leg> {
    use ObligationAccount::{FeesReceivable, Position, TaxesPayable};
    let ordering = r.payment.ordering();
    let total = r.amount.checked_add(r.fee).and_then(|s| s.checked_add(r.tax)).expect("checked when reserved");
    let headroom = Leg::Headroom {
        debit: HeadroomAccount::Integrante(ordering),
        credit: HeadroomAccount::Committed,
        amount: Headroom::of(total),
    };
    let owed = |credit, amount| Leg::Obligations { debit: Position(ordering), credit, amount };
    [headroom, owed(Position(r.beneficiary), r.amount), owed(FeesReceivable, r.fee), owed(TaxesPayable, r.tax)]
        .into_iter()
        .filter(|leg| leg.units() > 0)
        .collect()
}

pub fn constrained(legs: &[Leg]) -> Vec<AccountId> {
    let mut accounts: Vec<_> = legs.iter().flat_map(|l| [l.debit(), l.credit()]).filter(|a| a.no_overdraft()).collect();
    accounts.sort();
    accounts.dedup();
    accounts
}

fn refuse_overdraft(view: &impl View, draft: &Draft) -> Result<(), Refusal> {
    for account in constrained(&draft.legs) {
        let mut balance = view.balance(account);
        for leg in &draft.legs {
            balance.apply(account, &draft.kind, leg);
        }
        if balance.available() < 0 {
            return Err(Refusal::InsufficientHeadroom);
        }
    }
    Ok(())
}

impl Balance {
    fn apply(&mut self, account: AccountId, kind: &EntryKind, leg: &Leg) {
        let units = u128::from(leg.units());
        for (side, debit) in [(leg.debit(), true), (leg.credit(), false)] {
            if side != account {
                continue;
            }
            let (pending, posted) = match debit {
                true => (&mut self.debits_pending, &mut self.debits_posted),
                false => (&mut self.credits_pending, &mut self.credits_posted),
            };
            match kind {
                EntryKind::Reserved(_) => *pending += units,
                EntryKind::Posted(_) => {
                    *pending -= units;
                    *posted += units;
                }
                EntryKind::Voided(..) => *pending -= units,
                EntryKind::Transferred(..) => *posted += units,
            }
        }
    }
}

// Microseconds, because that is what Postgres stores.
pub fn next_commit_time(previous: Option<Timestamp>, now: Timestamp) -> Timestamp {
    let now = Timestamp::from_microsecond(now.as_microsecond()).expect("in range");
    match previous {
        Some(p) if now <= p => p + SignedDuration::from_micros(1),
        _ => now,
    }
}

impl Reservation {
    fn pending(r: &Reserve) -> Self {
        Self {
            payment: r.payment.clone(),
            beneficiary: r.beneficiary,
            amount: r.amount,
            fee: r.fee,
            tax: r.tax,
            state: ReservationState::Pending,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Replay {
    reservations: HashMap<PaymentId, Reservation>,
    transfers: HashMap<TransferId, Vec<Leg>>,
    balances: HashMap<AccountId, Balance>,
}

impl Replay {
    pub fn of<'a>(entries: impl IntoIterator<Item = &'a Entry>) -> Self {
        let mut replay = Self::default();
        entries.into_iter().for_each(|e| replay.apply(e));
        replay
    }

    pub fn apply(&mut self, entry: &Entry) {
        for leg in &entry.legs {
            let (debit, credit) = (leg.debit(), leg.credit());
            for account in std::iter::once(debit).chain((credit != debit).then_some(credit)) {
                self.balances.entry(account).or_default().apply(account, &entry.kind, leg);
            }
        }
        match &entry.kind {
            EntryKind::Reserved(r) => {
                self.reservations.insert(r.payment.clone(), Reservation::pending(r));
            }
            EntryKind::Transferred(id, _) => {
                self.transfers.insert(*id, entry.legs.clone());
            }
            EntryKind::Posted(p) => self.settle(p, ReservationState::Posted),
            EntryKind::Voided(p, reason) => self.settle(p, ReservationState::Voided(*reason)),
        }
    }

    fn settle(&mut self, payment: &PaymentId, state: ReservationState) {
        if let Some(r) = self.reservations.get_mut(payment) {
            r.state = state;
        }
    }

    pub fn positions(&self) -> Vec<crate::Position> {
        let mut positions: Vec<_> =
            self.balances.iter().map(|(&account, &balance)| crate::Position { account, balance }).collect();
        positions.sort_by_key(|p| p.account);
        positions
    }
}

impl View for Replay {
    fn reservation(&self, payment: &PaymentId) -> Option<Reservation> {
        self.reservations.get(payment).cloned()
    }

    fn transfer(&self, id: TransferId) -> Option<Vec<Leg>> {
        self.transfers.get(&id).cloned()
    }

    fn balance(&self, account: AccountId) -> Balance {
        self.balances.get(&account).copied().unwrap_or_default()
    }
}
