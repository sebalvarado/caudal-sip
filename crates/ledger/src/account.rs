use std::fmt;
use std::str::FromStr;

use caudal_types::{Bic, InvalidValue, Usd};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Book {
    Obligations,
    Headroom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObligationAccount {
    Position(Bic),
    FeesReceivable,
    TaxesPayable,
    Settled,
    Suspense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeadroomAccount {
    Integrante(Bic),
    Granted,
    Committed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccountId {
    Obligations(ObligationAccount),
    Headroom(HeadroomAccount),
}

impl AccountId {
    pub const fn book(self) -> Book {
        match self {
            Self::Obligations(_) => Book::Obligations,
            Self::Headroom(_) => Book::Headroom,
        }
    }

    pub const fn no_overdraft(self) -> bool {
        matches!(self, Self::Headroom(HeadroomAccount::Integrante(_)))
    }
}

impl From<ObligationAccount> for AccountId {
    fn from(a: ObligationAccount) -> Self {
        Self::Obligations(a)
    }
}

impl From<HeadroomAccount> for AccountId {
    fn from(a: HeadroomAccount) -> Self {
        Self::Headroom(a)
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use {HeadroomAccount as H, ObligationAccount as O};
        match self {
            Self::Obligations(O::Position(bic)) => write!(f, "ir:{bic}:posicion"),
            Self::Headroom(H::Integrante(bic)) => write!(f, "ir:{bic}:headroom"),
            Self::Obligations(O::FeesReceivable) => f.write_str("scheme:comisiones_por_cobrar"),
            Self::Obligations(O::TaxesPayable) => f.write_str("scheme:impuestos_por_pagar"),
            Self::Obligations(O::Settled) => f.write_str("scheme:liquidado"),
            Self::Obligations(O::Suspense) => f.write_str("scheme:suspenso"),
            Self::Headroom(H::Granted) => f.write_str("scheme:headroom_otorgado"),
            Self::Headroom(H::Committed) => f.write_str("scheme:headroom_comprometido"),
        }
    }
}

impl FromStr for AccountId {
    type Err = InvalidValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use {HeadroomAccount as H, ObligationAccount as O};
        Ok(match s {
            "scheme:comisiones_por_cobrar" => O::FeesReceivable.into(),
            "scheme:impuestos_por_pagar" => O::TaxesPayable.into(),
            "scheme:liquidado" => O::Settled.into(),
            "scheme:suspenso" => O::Suspense.into(),
            "scheme:headroom_otorgado" => H::Granted.into(),
            "scheme:headroom_comprometido" => H::Committed.into(),
            _ => match s.strip_prefix("ir:").and_then(|r| r.split_once(':')) {
                Some((bic, "posicion")) => O::Position(bic.parse()?).into(),
                Some((bic, "headroom")) => H::Integrante(bic.parse()?).into(),
                _ => return Err(InvalidValue::new("AccountId", "a name in the chart of accounts")),
            },
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Headroom(u64);

impl Headroom {
    pub const fn of(amount: Usd) -> Self {
        Self(amount.cents())
    }

    pub const fn units(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Balance {
    pub debits_pending: u128,
    pub debits_posted: u128,
    pub credits_pending: u128,
    pub credits_posted: u128,
}

impl Balance {
    pub fn available(&self) -> i128 {
        self.credits_posted as i128 - self.debits_posted as i128 - self.debits_pending as i128
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub account: AccountId,
    pub balance: Balance,
}
