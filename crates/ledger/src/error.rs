use std::{error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    InsufficientHeadroom,
    Conflict,
    NotReserved,
    NotTransferred,
    Irrevocable,
    Voided,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InsufficientHeadroom => "insufficient headroom",
            Self::Conflict => "conflicts with what is recorded under the same id",
            Self::NotReserved => "no reservation for this payment",
            Self::NotTransferred => "no transfer to reverse",
            Self::Irrevocable => "the payment is posted and irrevocable",
            Self::Voided => "the payment is voided",
        })
    }
}

#[derive(Debug)]
pub enum LedgerError {
    Refused(Refusal),
    // Never a refusal: the caller repeats the command.
    Unknown(Box<dyn error::Error + Send + Sync>),
}

impl From<Refusal> for LedgerError {
    fn from(r: Refusal) -> Self {
        Self::Refused(r)
    }
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Refused(r) => write!(f, "refused: {r}"),
            Self::Unknown(e) => write!(f, "outcome unknown: {e}"),
        }
    }
}

impl error::Error for LedgerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Refused(_) => None,
            Self::Unknown(e) => Some(e.as_ref()),
        }
    }
}
