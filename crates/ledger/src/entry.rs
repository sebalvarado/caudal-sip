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

    /// Swaps the sides. The amount stays. A reversal is this applied to every leg, so a caller cannot drop one.
    pub fn mirror(self) -> Self {
        match self {
            Self::Obligations { debit, credit, amount } => Self::Obligations { debit: credit, credit: debit, amount },
            Self::Headroom { debit, credit, amount } => Self::Headroom { debit: credit, credit: debit, amount },
        }
    }
}

/// What a transfer is for. The legs still say how it posts; this says which scheme event it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purpose {
    Headroom,
    Settlement,
    Suspense,
    Correction,
}

impl Purpose {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Headroom => "headroom",
            Self::Settlement => "settlement",
            Self::Suspense => "suspense",
            Self::Correction => "correction",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "headroom" => Self::Headroom,
            "settlement" => Self::Settlement,
            "suspense" => Self::Suspense,
            "correction" => Self::Correction,
            _ => return None,
        })
    }
}

/// Whether this call appended an entry. A replay found the books already holding the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Appended,
    Replay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applied<T> {
    pub value: T,
    pub effect: Effect,
}

impl<T> Applied<T> {
    pub fn replayed(&self) -> bool {
        self.effect == Effect::Replay
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
    pub purpose: Purpose,
    pub legs: Vec<Leg>,
    pub cause: Cause,
}

/// What the books already hold for a transfer id: the purpose and every leg.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedTransfer {
    pub purpose: Purpose,
    pub legs: Vec<Leg>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntryId {
    Reserved(PaymentId),
    Posted(PaymentId),
    Voided(PaymentId),
    Transfer(TransferId),
    Reversal(TransferId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    Reserved(Reserve),
    Posted(PaymentId),
    Voided(PaymentId, ReasonCode),
    Transferred { id: TransferId, purpose: Purpose, cause: Cause },
    Reversed { of: TransferId, cause: Cause },
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
            EntryKind::Transferred { id, .. } => EntryId::Transfer(*id),
            EntryKind::Reversed { of, .. } => EntryId::Reversal(*of),
        }
    }

    pub fn payment(&self) -> Option<&PaymentId> {
        match &self.kind {
            EntryKind::Reserved(r) => Some(&r.payment),
            EntryKind::Posted(p) | EntryKind::Voided(p, _) => Some(p),
            EntryKind::Transferred { .. } | EntryKind::Reversed { .. } => None,
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
