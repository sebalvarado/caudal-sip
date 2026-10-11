use caudal_ledger::MemLedger;
use caudal_ledger::testing::{clock, op, run};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    #[test]
    fn every_operation_sequence_keeps_the_invariants(ops in prop::collection::vec(op(), 1..60)) {
        pollster::block_on(run(&MemLedger::new(clock()), &ops));
    }
}
