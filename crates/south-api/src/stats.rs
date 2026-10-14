use std::collections::BTreeMap;
use std::time::Duration;

use jiff::SignedDuration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    Codec,
    Signature,
    Rules,
    Ledger,
}

impl Op {
    pub const ALL: [Self; 4] = [Self::Codec, Self::Signature, Self::Rules, Self::Ledger];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Codec => "codec",
            Self::Signature => "signature",
            Self::Rules => "rules",
            Self::Ledger => "ledger",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Summary {
    pub count: usize,
    pub p50: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub total: Duration,
}

impl Summary {
    pub fn per_second(&self) -> f64 {
        let secs = self.total.as_secs_f64();
        if secs == 0.0 { 0.0 } else { self.count as f64 / secs }
    }
}

fn summarize(samples: &[u64]) -> Option<Summary> {
    if samples.is_empty() {
        return None;
    }
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let at = |q: f64| Duration::from_nanos(sorted[((sorted.len() - 1) as f64 * q).round() as usize]);
    Some(Summary {
        count: sorted.len(),
        p50: at(0.50),
        p95: at(0.95),
        p99: at(0.99),
        total: Duration::from_nanos(sorted.iter().sum()),
    })
}

/// Real computation per hop, timed with a monotonic clock. A floor: no network, TLS or core banking.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    samples: BTreeMap<Op, Vec<u64>>,
    reserve_to_post: Vec<u64>,
}

impl Stats {
    pub fn record(&mut self, op: Op, elapsed: Duration) {
        self.samples.entry(op).or_default().push(elapsed.as_nanos() as u64);
    }

    pub fn record_reserve_to_post(&mut self, simulated: SignedDuration) {
        self.reserve_to_post.push(simulated.as_nanos().max(0) as u64);
    }

    pub fn summary(&self, op: Op) -> Option<Summary> {
        summarize(self.samples.get(&op).map_or(&[], Vec::as_slice))
    }

    pub fn reserve_to_post(&self) -> Option<Summary> {
        summarize(&self.reserve_to_post)
    }

    pub fn compute(&self) -> Duration {
        Duration::from_nanos(self.samples.values().flatten().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentiles_pick_ranked_samples() {
        let mut s = Stats::default();
        for n in 1..=100 {
            s.record(Op::Rules, Duration::from_nanos(n));
        }
        let r = s.summary(Op::Rules).unwrap();
        assert_eq!(
            (r.count, r.p50, r.p95, r.p99),
            (100, Duration::from_nanos(51), Duration::from_nanos(95), Duration::from_nanos(99))
        );
        assert!(s.summary(Op::Codec).is_none());
    }
}
