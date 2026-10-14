use std::fmt;

use caudal_ledger::{Headroom, HeadroomAccount, Leg};
use caudal_types::{Bic, Usd};

fn percent_half_up(cents: u64, numerator: u64, denominator: u64) -> Usd {
    let scaled = u128::from(cents) * u128::from(numerator);
    let d = u128::from(denominator);
    Usd::from_cents(((scaled + d / 2) / d) as u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ceiling {
    pub encaje: Usd,
    pub acceptable: Usd,
}

impl Ceiling {
    /// Each step rounds half-up to the cent, as in the catálogo's worked example.
    pub fn compute(deposits: Usd, encaje_bp: u32, factor_pct: u32) -> Self {
        let encaje = percent_half_up(deposits.cents(), u64::from(encaje_bp), 10_000);
        Self { encaje, acceptable: percent_half_up(encaje.cents(), u64::from(factor_pct), 100) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Light {
    Green,
    Yellow,
    Orange,
    Red,
}

impl Light {
    pub fn of(ceiling: Usd, available: i128) -> Self {
        let ceiling = i128::from(ceiling.cents());
        if ceiling == 0 || available <= 0 {
            return Self::Red;
        }
        match (ceiling - available) * 100 / ceiling {
            ..75 => Self::Green,
            75..90 => Self::Yellow,
            90..100 => Self::Orange,
            _ => Self::Red,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Green => "verde",
            Self::Yellow => "amarillo",
            Self::Orange => "naranja",
            Self::Red => "rojo",
        }
    }
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adjustment {
    pub legs: Vec<Leg>,
    pub pending_reduction: Usd,
}

/// Moves an integrante's granted headroom from `granted` to `target`. A decrease never takes more than is
/// available: existing commitments stand, and the rest waits as a pending reduction.
pub fn adjust(bic: Bic, granted: Usd, target: Usd, available: i128) -> Adjustment {
    if target >= granted {
        let increase = target.checked_sub(granted).expect("target is at least granted");
        return Adjustment { legs: grant(bic, increase), pending_reduction: Usd::ZERO };
    }
    let decrease = granted.checked_sub(target).expect("granted exceeds target");
    apply_pending(bic, decrease, available)
}

/// Applies as much of a pending reduction as the integrante's available headroom now allows.
pub fn apply_pending(bic: Bic, pending: Usd, available: i128) -> Adjustment {
    let now = Usd::from_cents(u64::try_from(available.clamp(0, i128::from(pending.cents()))).expect("clamped"));
    let leg = Leg::Headroom {
        debit: HeadroomAccount::Integrante(bic),
        credit: HeadroomAccount::Granted,
        amount: Headroom::of(now),
    };
    Adjustment {
        legs: (now > Usd::ZERO).then_some(leg).into_iter().collect(),
        pending_reduction: pending.checked_sub(now).expect("at most pending"),
    }
}

fn grant(bic: Bic, amount: Usd) -> Vec<Leg> {
    let leg = Leg::Headroom {
        debit: HeadroomAccount::Granted,
        credit: HeadroomAccount::Integrante(bic),
        amount: Headroom::of(amount),
    };
    (amount > Usd::ZERO).then_some(leg).into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use caudal_ledger::testing::{bic, cause, clock, payment};
    use caudal_ledger::{AsOf, Ledger, MemLedger, Reserve, Transfer, TransferId};
    use pollster::block_on;
    use proptest::prelude::*;
    use uuid::Uuid;

    fn usd(s: &str) -> Usd {
        s.parse().unwrap()
    }

    #[test]
    fn reproduces_the_worked_example() {
        let c = Ceiling::compute(usd("11141.71"), 500, 25);
        assert_eq!((c.encaje, c.acceptable), (usd("557.09"), usd("139.27")));
    }

    #[test]
    fn traffic_light_thresholds_match_the_example() {
        let ceiling = usd("139.27");
        let at = |committed: &str| Light::of(ceiling, i128::from(ceiling.cents() - usd(committed).cents()));
        assert_eq!(at("103.06"), Light::Green);
        assert_eq!(at("104.46"), Light::Yellow);
        assert_eq!(at("125.35"), Light::Orange);
        assert_eq!(at("139.27"), Light::Red);
        assert_eq!(Light::of(Usd::ZERO, 0), Light::Red);
    }

    fn apply(ledger: &MemLedger, n: u128, legs: Vec<Leg>) {
        if !legs.is_empty() {
            block_on(ledger.transfer(Transfer {
                id: TransferId(Uuid::from_u128(n)),
                purpose: caudal_ledger::Purpose::Headroom,
                legs,
                cause: cause(n as u8),
            }))
            .unwrap();
        }
    }

    fn available(ledger: &MemLedger) -> i128 {
        block_on(ledger.balance(HeadroomAccount::Integrante(bic(0)).into(), AsOf::Latest)).unwrap().available()
    }

    #[test]
    fn a_decrease_below_commitments_waits_as_a_pending_reduction() {
        let ledger = MemLedger::new(clock());
        apply(&ledger, 1, adjust(bic(0), Usd::ZERO, usd("100.00"), 0).legs);
        let r = Reserve {
            payment: payment(0, 1),
            beneficiary: bic(1),
            amount: usd("80.00"),
            fee: Usd::ZERO,
            tax: Usd::ZERO,
            cause: cause(2),
        };
        block_on(ledger.reserve(r)).unwrap();
        let cut = adjust(bic(0), usd("100.00"), usd("10.00"), available(&ledger));
        assert_eq!(cut.pending_reduction, usd("70.00"));
        apply(&ledger, 3, cut.legs);
        assert_eq!(available(&ledger), 0, "the commitment stands, nothing new fits");
        block_on(ledger.void(payment(0, 1), caudal_types::ReasonCode::Dt01)).unwrap();
        let rest = apply_pending(bic(0), cut.pending_reduction, available(&ledger));
        assert_eq!(rest.pending_reduction, Usd::ZERO);
        apply(&ledger, 4, rest.legs);
        assert_eq!(available(&ledger), 1_000);
    }

    proptest! {
        #[test]
        fn adjustments_reach_the_target_or_leave_the_gap_pending(
            granted in 0u64..1_000_000, target in 0u64..1_000_000, committed in 0u64..1_000_000,
        ) {
            let committed = committed.min(granted);
            let available = i128::from(granted - committed);
            let a = adjust(bic(0), Usd::from_cents(granted), Usd::from_cents(target), available);
            let moved: i128 = a.legs.iter().map(|l| match *l {
                Leg::Headroom { debit: HeadroomAccount::Granted, amount, .. } => i128::from(amount.units()),
                Leg::Headroom { amount, .. } => -i128::from(amount.units()),
                Leg::Obligations { .. } => unreachable!(),
            }).sum();
            prop_assert!(available + moved >= 0);
            prop_assert_eq!(i128::from(granted) + moved - i128::from(a.pending_reduction.cents()), i128::from(target));
        }
    }
}
