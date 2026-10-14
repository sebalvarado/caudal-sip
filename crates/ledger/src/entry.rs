use caudal_types::{Bic, PaymentId, ReasonCode, Usd};
use jiff::Timestamp;
use uuid::Uuid;

use crate::{AccountId, Headroom, HeadroomAccount, ObligationAccount};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Leg {
    Obligations { debit: ObligationAccount, credit: ObligationAccount, amount: Usd },
    Headroom { debit: HeadroomAccount, credit: HeadroomAccount, amount: Headroom },
}

impl Leg {
    pub fn debit(&self) -> AccountId {
        match *self {
            Self::Obligations { debit, .. } => debit.into(),
            Self::Headroom { debit, .. } => debit.into(),
        }
    }

    pub fn credit(&self) -> AccountId {
        match *self {
            Self::Obligations { credit, .. } => credit.into(),
            Self::Headroom { credit, .. } => credit.into(),
        }
    }

    pub fn units(&self) -> u64 {
        match *self {
            Self::Obligations { amount, .. } => amount.cents(),
            Self::Headroom { amount, .. } => amount.units(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cause {
    pub event: Uuid,
    pub digest: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reserve {
    pub payment: PaymentId,
    pub beneficiary: Bic,
    pub amount: Usd,
    pub fee: Usd,
    pub tax: Usd,
    pub cause: Cause,
}

impl Reserve {
    pub fn total(&self) -> Option<Usd> {
        self.amount.checked_add(self.fee)?.checked_add(self.tax)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservationState {
    Pending,
    Posted,
    Voided(ReasonCode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reservation {
    pub payment: PaymentId,
    pub beneficiary: Bic,
    pub amount: Usd,
    pub fee: Usd,
    pub tax: Usd,
    pub state: ReservationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransferId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    pub id: TransferId,
    pub legs: Vec<Leg>,
    pub cause: Cause,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryId {
    Reserved(PaymentId),
    Posted(PaymentId),
    Voided(PaymentId),
    Transfer(TransferId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Reserved(Reserve),
    Posted(PaymentId),
    Voided(PaymentId, ReasonCode),
    Transferred(TransferId, Cause),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draft {
    pub kind: EntryKind,
    pub legs: Vec<Leg>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub at: Timestamp,
    pub kind: EntryKind,
    pub legs: Vec<Leg>,
}

impl Entry {
    pub fn id(&self) -> EntryId {
        match &self.kind {
            EntryKind::Reserved(r) => EntryId::Reserved(r.payment.clone()),
            EntryKind::Posted(p) => EntryId::Posted(p.clone()),
            EntryKind::Voided(p, _) => EntryId::Voided(p.clone()),
            EntryKind::Transferred(id, _) => EntryId::Transfer(*id),
        }
    }

    pub fn payment(&self) -> Option<&PaymentId> {
        match &self.kind {
            EntryKind::Reserved(r) => Some(&r.payment),
            EntryKind::Posted(p) | EntryKind::Voided(p, _) => Some(p),
            EntryKind::Transferred(..) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsOf {
    Latest,
    At(Timestamp),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryQuery {
    All,
    Payment(PaymentId),
}
