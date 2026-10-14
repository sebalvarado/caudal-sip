use std::fmt;
use std::str::FromStr;

use crate::script::{self, Report, Run};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    Single,
    Payroll,
    CutOff,
    National,
}

impl Scenario {
    pub const ALL: [Self; 4] = [Self::Single, Self::Payroll, Self::CutOff, Self::National];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Payroll => "payroll",
            Self::CutOff => "cutoff",
            Self::National => "national",
        }
    }

    pub const fn source(self) -> &'static str {
        match self {
            Self::Single => SINGLE,
            Self::Payroll => PAYROLL,
            Self::CutOff => CUTOFF,
            Self::National => NATIONAL,
        }
    }

    pub fn title(self) -> &'static str {
        self.source().lines().next().and_then(|l| l.strip_prefix("# ")).unwrap_or_default()
    }

    pub fn run(self, seed: Option<u64>) -> Run {
        script::run(self.source(), seed).expect("the built-in scenarios parse")
    }

    pub fn report(self, seed: Option<u64>) -> Report {
        self.run(seed).report
    }
}

impl FromStr for Scenario {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|x| x.name() == s)
            .ok_or_else(|| format!("no scenario {s:?}; try single, payroll, cutoff or national"))
    }
}

impl fmt::Display for Scenario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

const SINGLE: &str = "\
# Un pago entre redes y su liquidación
# coop-beta, en la red beta, le paga a una panadería de banco-alfa, en la red alfa.

at 09:00 pay coop-beta llave-02 25.40
show last
settle
";

const PAYROLL: &str = "\
# Día de pago de sueldos en una cooperativa del segmento 3
# coop-beta paga 300 sueldos con un techo de USD 60.000. Lo que no alcanza se
# reintenta después de cada liquidación, cuando el cupo vuelve.

seed 7
at 08:00 traffic 600 until 12:00
at 08:00 repeat 300 every 18s: pay coop-beta llave-02,llave-04,llave-05 180..520
at 11:00 settle
wait 1m
retry
at 15:00 settle
wait 1m
retry
";

const CUTOFF: &str = "\
# La media hora antes del cierre
# 3.000 pagos entre las 16:30 y las 17:00, y una liquidación a las 17:30.

seed 7
at 16:30 traffic 3000 until 17:00
at 17:00 network
at 17:30 settle
";

const NATIONAL: &str = "\
# Un día de pagos de todo un país
# Tráfico aleatorio entre las cuatro instituciones, la nómina de coop-beta a las
# 09:00 y tres liquidaciones.

seed 7
at 08:00 traffic 20000 until 17:30
at 09:00 repeat 300 every 12s: pay coop-beta llave-02,llave-04,llave-05 180..520
at 11:00 settle
wait 1m
retry
at 15:00 settle
wait 1m
retry
at 17:30 settle
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_scenario_parses_and_has_a_title() {
        for s in Scenario::ALL {
            assert!(!s.title().is_empty(), "{s}");
            assert!(script::check(s.source()).is_ok(), "{s}");
        }
    }

    #[test]
    fn the_single_scenario_settles_one_payment() {
        let r = Scenario::Single.report(None);
        assert_eq!((r.payments, r.settled), (1, 1));
    }

    #[test]
    fn payroll_hits_the_ceiling_and_recovers_after_settlement() {
        let t = Scenario::Payroll.report(None).tracked.unwrap();
        assert_eq!(t.items, 300);
        assert!(t.blocked_am04 > 0, "a segment-3 payroll exceeds the ceiling between settlements");
        assert!(t.paid_after_retry > 0, "a settlement restores headroom");
        assert_eq!(t.paid_first_try + t.blocked_am04, t.items);
    }
}
