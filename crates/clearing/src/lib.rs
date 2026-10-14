use std::collections::{BTreeMap, HashMap, HashSet};

use caudal_ledger::core::{Replay, View};
use caudal_ledger::{
    Cause, Entry, EntryKind, Headroom, HeadroomAccount, Leg, ObligationAccount, Purpose, Reserve, Transfer, TransferId,
};
use caudal_types::{Bic, PaymentId, Usd};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Malla {
    /// Positive: the integrante is owed. Negative: it owes.
    pub nets: BTreeMap<Bic, i128>,
    pub bilateral: BTreeMap<(Bic, Bic), Usd>,
    pub release: BTreeMap<Bic, Headroom>,
    pub payments: Vec<PaymentId>,
}

impl Malla {
    pub fn is_empty(&self) -> bool {
        self.payments.is_empty() && self.nets.values().all(|n| *n == 0)
    }

    /// What the payments add up to before netting.
    pub fn gross(&self) -> Usd {
        Usd::from_cents(self.bilateral.values().map(|u| u.cents()).sum())
    }

    /// What moves at settlement: the sum of the positive nets.
    pub fn total(&self) -> Usd {
        Usd::from_cents(self.nets.values().filter(|n| **n > 0).map(|n| *n as u64).sum())
    }
}

fn reversed(entries: &[Entry]) -> HashSet<TransferId> {
    entries
        .iter()
        .filter_map(|e| match e.kind {
            EntryKind::Reversed { of, .. } => Some(of),
            _ => None,
        })
        .collect()
}

fn is_settlement(entry: &Entry, reversed: &HashSet<TransferId>) -> bool {
    matches!(entry.kind, EntryKind::Transferred { id, purpose: Purpose::Settlement, .. } if !reversed.contains(&id))
}

/// The multilateral malla of every payment posted since the last settlement. Bilateral totals are for
/// reporting only.
pub fn malla(entries: &[Entry]) -> Malla {
    let reversed = reversed(entries);
    let start = entries.iter().rposition(|e| is_settlement(e, &reversed)).map_or(0, |i| i + 1);
    let reserves: HashMap<&PaymentId, &Reserve> = entries
        .iter()
        .filter_map(|e| match &e.kind {
            EntryKind::Reserved(r) => Some((&r.payment, r)),
            _ => None,
        })
        .collect();
    let mut m = Malla::default();
    for entry in &entries[start..] {
        let EntryKind::Posted(p) = &entry.kind else { continue };
        let r = reserves[p];
        let ordering = p.ordering();
        let pair = m.bilateral.entry((ordering, r.beneficiary)).or_default();
        *pair = pair.checked_add(r.amount).expect("a malla fits in u64 cents");
        let total = r.total().expect("checked when reserved");
        let release = m.release.entry(ordering).or_default();
        *release = Headroom::of(Usd::from_cents(release.units() + total.cents()));
        m.payments.push(p.clone());
    }
    let replay = Replay::of(entries);
    for position in replay.positions() {
        if let caudal_ledger::AccountId::Obligations(ObligationAccount::Position(bic)) = position.account {
            let b = replay.balance(position.account);
            let net = b.credits_posted as i128 - b.debits_posted as i128;
            if net != 0 {
                m.nets.insert(bic, net);
            }
        }
    }
    m
}

/// The entry that settles a malla: positions close against `Settled`, and committed headroom returns to
/// each ordering integrante. `None` when there is nothing to settle.
pub fn settlement(malla: &Malla, id: TransferId, cause: Cause) -> Option<Transfer> {
    let mut legs = Vec::new();
    for (&bic, &net) in &malla.nets {
        let amount = Usd::from_cents(net.unsigned_abs() as u64);
        let position = ObligationAccount::Position(bic);
        legs.push(match net > 0 {
            true => Leg::Obligations { debit: position, credit: ObligationAccount::Settled, amount },
            false => Leg::Obligations { debit: ObligationAccount::Settled, credit: position, amount },
        });
    }
    for (&bic, &amount) in &malla.release {
        legs.push(Leg::Headroom {
            debit: HeadroomAccount::Committed,
            credit: HeadroomAccount::Integrante(bic),
            amount,
        });
    }
    (!legs.is_empty()).then_some(Transfer { id, purpose: Purpose::Settlement, legs, cause })
}

#[cfg(test)]
mod tests {
    use super::*;
    use caudal_ledger::testing::{bic, cause, clock, payment};
    use caudal_ledger::{AccountId, AsOf, EntryQuery, Ledger, MemLedger};
    use caudal_types::ReasonCode;
    use pollster::block_on;
    use proptest::prelude::*;
    use uuid::Uuid;

    const FUNDED: u64 = 100_000;

    fn funded() -> MemLedger {
        let ledger = MemLedger::new(clock());
        let legs = (0..3)
            .map(|i| Leg::Headroom {
                debit: HeadroomAccount::Granted,
                credit: HeadroomAccount::Integrante(bic(i)),
                amount: Headroom::of(Usd::from_cents(FUNDED)),
            })
            .collect();
        block_on(ledger.transfer(Transfer {
            id: TransferId(Uuid::from_u128(1)),
            purpose: Purpose::Headroom,
            legs,
            cause: cause(1),
        }))
        .unwrap();
        ledger
    }

    fn pay(ledger: &MemLedger, from: usize, to: usize, n: u8, cents: u64) -> PaymentId {
        let p = payment(from, n);
        let r = Reserve {
            payment: p.clone(),
            beneficiary: bic(to),
            amount: Usd::from_cents(cents),
            fee: Usd::ZERO,
            tax: Usd::ZERO,
            cause: cause(n),
        };
        block_on(ledger.reserve(r)).unwrap();
        p
    }

    fn settle(ledger: &MemLedger, n: u128) -> Malla {
        let entries = block_on(ledger.entries(EntryQuery::All)).unwrap();
        let m = malla(&entries);
        if let Some(t) = settlement(&m, TransferId(Uuid::from_u128(1_000 + n)), cause(200)) {
            block_on(ledger.transfer(t)).unwrap();
        }
        m
    }

    fn available(ledger: &MemLedger, i: usize) -> i128 {
        block_on(ledger.balance(AccountId::Headroom(HeadroomAccount::Integrante(bic(i))), AsOf::Latest))
            .unwrap()
            .available()
    }

    #[test]
    fn nets_multilaterally_and_reports_bilaterally() {
        let ledger = funded();
        for (n, (from, to, cents)) in [(0, 1, 5_000), (1, 0, 3_000), (1, 2, 1_000), (2, 0, 500)].into_iter().enumerate()
        {
            let p = pay(&ledger, from, to, n as u8, cents);
            block_on(ledger.post(p)).unwrap();
        }
        let m = settle(&ledger, 1);
        assert_eq!(m.nets[&bic(0)], -5_000 + 3_000 + 500);
        assert_eq!(m.nets[&bic(1)], 5_000 - 3_000 - 1_000);
        assert_eq!(m.nets[&bic(2)], 1_000 - 500);
        assert_eq!(m.nets.values().sum::<i128>(), 0);
        assert_eq!(m.bilateral[&(bic(0), bic(1))], Usd::from_cents(5_000));
        assert_eq!(m.payments.len(), 4);
        assert_eq!(m.gross(), Usd::from_cents(9_500));
        assert_eq!(m.total(), Usd::from_cents(1_500), "only the nets move");
        assert!(settle(&ledger, 2).is_empty(), "a settled malla is not settled again");
        for i in 0..3 {
            assert_eq!(available(&ledger, i), i128::from(FUNDED), "headroom returns on settlement");
        }
    }

    #[test]
    fn reversing_a_settlement_reopens_the_malla() {
        let ledger = funded();
        let p = pay(&ledger, 0, 1, 1, 2_000);
        block_on(ledger.post(p.clone())).unwrap();
        settle(&ledger, 1);
        assert!(malla(&block_on(ledger.entries(EntryQuery::All)).unwrap()).payments.is_empty());
        block_on(ledger.reverse(TransferId(Uuid::from_u128(1_001)), cause(9))).unwrap();
        let reopened = malla(&block_on(ledger.entries(EntryQuery::All)).unwrap());
        assert_eq!(reopened.payments, vec![p]);
    }

    #[test]
    fn a_replayed_instruction_does_not_double_post() {
        let ledger = funded();
        let p = pay(&ledger, 0, 1, 1, 2_000);
        block_on(ledger.post(p)).unwrap();
        let entries = block_on(ledger.entries(EntryQuery::All)).unwrap();
        let t = settlement(&malla(&entries), TransferId(Uuid::from_u128(9)), cause(9)).unwrap();
        block_on(ledger.transfer(t.clone())).unwrap();
        block_on(ledger.transfer(t)).unwrap();
        assert_eq!(block_on(ledger.entries(EntryQuery::All)).unwrap().len(), entries.len() + 1);
    }

    proptest! {
        #[test]
        fn settlement_zeroes_positions_and_releases_only_what_was_posted(
            flows in prop::collection::vec((0usize..3, 0usize..3, 1u64..2_000, 0u8..3), 1..30),
        ) {
            let ledger = funded();
            let mut pending = [0u64; 3];
            for (n, (from, to, cents, outcome)) in flows.into_iter().enumerate() {
                if from == to { continue; }
                if available(&ledger, from) < i128::from(cents) { continue; }
                let p = pay(&ledger, from, to, n as u8, cents);
                match outcome {
                    0 => { block_on(ledger.post(p)).unwrap(); }
                    1 => { block_on(ledger.void(p, ReasonCode::Ac01)).unwrap(); }
                    _ => pending[from] += cents,
                }
            }
            settle(&ledger, 1);
            let entries = block_on(ledger.entries(EntryQuery::All)).unwrap();
            prop_assert!(malla(&entries).nets.is_empty());
            for (i, held) in pending.iter().enumerate() {
                prop_assert_eq!(available(&ledger, i), i128::from(FUNDED - held));
            }
        }
    }
}
