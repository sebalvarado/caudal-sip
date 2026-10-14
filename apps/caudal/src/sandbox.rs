use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use caudal_directory::{AccountType, Credencial, Directory, DirectoryError, Llave, Resolution};
use caudal_ledger::{AccountId, AsOf, HeadroomAccount, Ledger, MemLedger};
use caudal_msgplane::SimClock;
use caudal_payments::{Answer, Deadlines, Integrante, Message, Order, Party, Red, Role, Script, Sip};
use caudal_registry::{Integrante as Member, Red as RedInfo, Registry, Sector, Status};
use caudal_risk::{Ceiling, Light};
use caudal_south_api::{Hop, Network, Row, Settlement};
use caudal_types::{Bic, LlaveType, PaymentId, PaymentState, ReasonCode, Usd, to_scheme_time};
use jiff::{SignedDuration, Timestamp};

pub const SIP: &str = "SIPCECEQ";

pub struct RedSeed {
    pub id: &'static str,
    pub bic: &'static str,
    pub name: &'static str,
}

pub const REDES: [RedSeed; 2] = [
    RedSeed { id: "red-alfa", bic: "RALFECEQ", name: "Red Alfa" },
    RedSeed { id: "red-beta", bic: "RBETECEQ", name: "Red Beta" },
];

pub struct InstitutionSeed {
    pub id: &'static str,
    pub bic: &'static str,
    pub name: &'static str,
    pub red: &'static str,
    pub sector: Sector,
    pub deposits: &'static str,
    pub customer: &'static str,
}

pub const INSTITUTIONS: [InstitutionSeed; 4] = [
    InstitutionSeed {
        id: "banco-alfa",
        bic: "BALFECEQ",
        name: "Banco Alfa",
        red: "red-alfa",
        sector: Sector::Financial,
        deposits: "400000000.00",
        customer: "María Elena Ejemplo Ficticia",
    },
    InstitutionSeed {
        id: "mutual-delta",
        bic: "MDELECEQ",
        name: "Mutualista Delta",
        red: "red-alfa",
        sector: Sector::Mutualista,
        deposits: "120000000.00",
        customer: "Rosa Amelia Muestra Inventada",
    },
    InstitutionSeed {
        id: "coop-beta",
        bic: "CBETECEQ",
        name: "Cooperativa Beta",
        red: "red-beta",
        sector: Sector::CooperativeSegment3,
        deposits: "6000000.00",
        customer: "Luis Alberto Prueba Simulado",
    },
    InstitutionSeed {
        id: "fintech-gamma",
        bic: "FGAMECEQ",
        name: "Fintech Gamma",
        red: "red-beta",
        sector: Sector::Sedpe,
        deposits: "3000000.00",
        customer: "Carla Sofía Demo Ficción",
    },
];

pub struct LlaveSeed {
    pub id: &'static str,
    pub kind: LlaveType,
    pub value: &'static str,
    pub holder: &'static str,
    pub account: &'static str,
    pub institution: &'static str,
}

/// Account numbers ending in 99 are blocked and in 98 closed, so the mocks can reject with a Tabla 12 code.
pub const LLAVES: [LlaveSeed; 8] = [
    LlaveSeed {
        id: "llave-01",
        kind: LlaveType::Celular,
        value: "0990000001",
        holder: "Julio César Tienda Andrade",
        account: "4400000001",
        institution: "coop-beta",
    },
    LlaveSeed {
        id: "llave-02",
        kind: LlaveType::Correo,
        value: "panaderia@ejemplo.ec",
        holder: "Panadería La Ficticia",
        account: "1100000002",
        institution: "banco-alfa",
    },
    LlaveSeed {
        id: "llave-03",
        kind: LlaveType::Identificacion,
        value: "0000000003",
        holder: "Ana Lucía Ejemplo Ruiz",
        account: "7700000003",
        institution: "fintech-gamma",
    },
    LlaveSeed {
        id: "llave-04",
        kind: LlaveType::Alfanumerico,
        value: "FERIASUR",
        holder: "Feria del Sur Inventada",
        account: "5500000004",
        institution: "mutual-delta",
    },
    LlaveSeed {
        id: "llave-05",
        kind: LlaveType::Celular,
        value: "0990000005",
        holder: "Pedro José Muestra León",
        account: "1100000005",
        institution: "banco-alfa",
    },
    LlaveSeed {
        id: "llave-06",
        kind: LlaveType::Correo,
        value: "taller@ejemplo.ec",
        holder: "Taller Mecánico Simulado",
        account: "4400000006",
        institution: "coop-beta",
    },
    LlaveSeed {
        id: "llave-07",
        kind: LlaveType::Celular,
        value: "0990000007",
        holder: "Cuenta Bloqueada Ficticia",
        account: "4400000099",
        institution: "coop-beta",
    },
    LlaveSeed {
        id: "llave-08",
        kind: LlaveType::Alfanumerico,
        value: "CERRADA",
        holder: "Cuenta Cerrada Ficticia",
        account: "1100000098",
        institution: "banco-alfa",
    },
];

pub fn bic(s: &str) -> Bic {
    s.parse().expect("seed BICs are valid")
}

pub fn institution(id: &str) -> Option<&'static InstitutionSeed> {
    INSTITUTIONS.iter().find(|i| i.id == id)
}

pub fn institution_by_bic(b: Bic) -> Option<&'static InstitutionSeed> {
    INSTITUTIONS.iter().find(|i| i.bic == b.as_str())
}

fn red_bic(id: &str) -> Bic {
    bic(REDES.iter().find(|r| r.id == id).expect("seeded red").bic)
}

pub fn ceiling(seed: &InstitutionSeed) -> Ceiling {
    let (encaje, factor) = seed.sector.liquidity().unwrap_or((0, 0));
    Ceiling::compute(seed.deposits.parse().expect("seeded deposits"), encaje, factor)
}

fn core_banking() -> Script {
    Script::new(|o: &Order| match &o.creditor.account {
        a if a.ends_with("99") => Answer::Reject(ReasonCode::Ac06),
        a if a.ends_with("98") => Answer::Reject(ReasonCode::Ac04),
        _ => Answer::Accept,
    })
}

pub fn start_of_day() -> Timestamp {
    "2026-11-30T13:00:00Z".parse().expect("valid")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandboxError {
    UnknownInstitution(String),
    UnknownLlave(String),
    Directory(DirectoryError),
}

impl fmt::Display for SandboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownInstitution(i) => {
                write!(f, "no institution {i:?}; try banco-alfa, mutual-delta, coop-beta or fintech-gamma")
            }
            Self::UnknownLlave(l) => write!(f, "no llave {l:?}; `llave list` shows the seeded ones"),
            Self::Directory(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SandboxError {}

#[derive(Debug, Clone, PartialEq)]
pub struct InstitutionView {
    pub id: &'static str,
    pub name: &'static str,
    pub bic: Bic,
    pub red: &'static str,
    pub sector: &'static str,
    pub ceiling: Usd,
    pub available: i128,
    pub light: Light,
    pub net: i128,
}

impl InstitutionView {
    pub fn used(&self) -> i128 {
        i128::from(self.ceiling.cents()) - self.available
    }

    /// Truncated to two decimals, so it never reads 100 while headroom remains.
    pub fn used_percent(&self) -> f64 {
        (self.used() * 10_000 / i128::from(self.ceiling.cents().max(1))) as f64 / 100.0
    }
}

pub struct Sandbox {
    net: Network<MemLedger>,
    registry: Registry,
    directory: Directory,
    ceilings: BTreeMap<&'static str, Usd>,
    sequence: u32,
}

impl Sandbox {
    /// The seeded network with a short history: payments in each terminal state, one of them por conciliar.
    pub fn new() -> Self {
        let mut s = Self::empty(start_of_day(), SignedDuration::from_millis(4));
        s.history();
        s
    }

    pub fn empty(start: Timestamp, latency: SignedDuration) -> Self {
        Self::with_ceilings(start, latency, &[])
    }

    /// An empty network where the listed institutions get the given ceiling instead of their Tabla 19 one.
    pub fn with_ceilings(start: Timestamp, latency: SignedDuration, overrides: &[(&'static str, Usd)]) -> Self {
        let ceilings: BTreeMap<&'static str, Usd> = INSTITUTIONS
            .iter()
            .map(|i| {
                let own = overrides.iter().rev().find(|(id, _)| *id == i.id).map(|(_, c)| *c);
                (i.id, own.unwrap_or_else(|| ceiling(i).acceptable))
            })
            .collect();
        let mut registry = Registry::default();
        for r in &REDES {
            registry.add_red(RedInfo { bic: bic(r.bic), name: r.name.into() }).expect("unique seed");
        }
        for i in &INSTITUTIONS {
            let member = Member {
                bic: bic(i.bic),
                name: i.name.into(),
                sector: i.sector,
                red: red_bic(i.red),
                status: Status::Active,
            };
            registry.add_integrante(member).expect("unique seed");
        }
        let mut directory = Directory::default();
        for l in &LLAVES {
            let owner = institution(l.institution).expect("seeded institution");
            let credencial = Credencial {
                holder: l.holder.into(),
                document: format!("00000000{:02}", l.id[6..].parse::<u8>().expect("numbered")),
                account: l.account.into(),
                account_type: AccountType::Ahorros,
                integrante: bic(owner.bic),
            };
            let llave = Llave::new(l.kind, l.value).expect("seed llaves are valid");
            directory.register(&registry, llave, credencial).expect("unique seed");
        }
        let d = Deadlines::default();
        let mut roles = vec![Role::Sip(Sip::new(bic(SIP), registry.clone(), d))];
        for r in &REDES {
            let members = INSTITUTIONS.iter().filter(|i| i.red == r.id).map(|i| bic(i.bic));
            roles.push(Role::Red(Red::new(bic(r.bic), bic(SIP), members, d)));
        }
        for i in &INSTITUTIONS {
            roles.push(Role::Integrante(Integrante::new(bic(i.bic), red_bic(i.red), d, core_banking())));
        }
        let clock = SimClock::starting_at(start);
        let ledger_clock = clock.clone();
        let ledger = MemLedger::new(move || ledger_clock.now());
        let mut net = Network::new(ledger, clock, roles, bic(SIP), b"caudal-sandbox", latency);
        for i in &INSTITUTIONS {
            let target = ceilings[i.id];
            net.fund(caudal_risk::adjust(bic(i.bic), Usd::ZERO, target, 0).legs).expect("funding a fresh ledger");
        }
        Self { net, registry, directory, ceilings, sequence: 0 }
    }

    fn history(&mut self) {
        let pay = |s: &mut Self, from, llave, amount: &str| {
            s.pay(from, llave, amount.parse().expect("valid")).expect("seeded")
        };
        pay(self, "banco-alfa", "llave-01", "25.40");
        pay(self, "fintech-gamma", "llave-02", "18.75");
        self.settle();
        pay(self, "coop-beta", "llave-04", "12.00");
        pay(self, "banco-alfa", "llave-07", "40.00");
        pay(self, "mutual-delta", "llave-08", "9.90");
        pay(self, "fintech-gamma", "llave-05", "16000.00");
        let unconfirmed = self.next_id("banco-alfa");
        let red_beta = bic("RBETECEQ");
        let target = unconfirmed.clone();
        self.net.add_fault(Arc::new(move |from, to, m| {
            from == red_beta
                && to == bic(SIP)
                && matches!(m, Message::Report(r) if r.id == target && r.state != PaymentState::Rcvd)
        }));
        pay(self, "banco-alfa", "llave-06", "33.10");
        self.net.clear_faults();
    }

    pub fn network(&self) -> &Network<MemLedger> {
        &self.net
    }

    pub fn network_mut(&mut self) -> &mut Network<MemLedger> {
        &mut self.net
    }

    pub fn now(&self) -> Timestamp {
        self.net.now()
    }

    pub fn llave(&self, key: &str) -> Result<Llave, SandboxError> {
        if let Some(seed) = LLAVES.iter().find(|l| l.id == key || l.value == key) {
            return Ok(Llave::new(seed.kind, seed.value).expect("seed llaves are valid"));
        }
        LlaveType::ALL
            .into_iter()
            .find_map(|kind| Llave::new(kind, key).ok().filter(|l| self.directory.credencial(l).is_ok()))
            .ok_or_else(|| SandboxError::UnknownLlave(key.into()))
    }

    pub fn resolve(&self, key: &str) -> Result<Resolution, SandboxError> {
        let llave = self.llave(key)?;
        self.directory.resolve(&self.registry, &llave).map_err(SandboxError::Directory)
    }

    fn next_id(&self, from: &str) -> PaymentId {
        let seed = institution(from).expect("checked by caller");
        let stamp = to_scheme_time(self.now()).strftime("%d%m%Y%H%M%S");
        PaymentId::new(bic(seed.bic), &format!("{}{stamp}{:06}", seed.bic, self.sequence + 1)).expect("well formed")
    }

    pub fn order(&mut self, from: &str, key: &str, amount: Usd) -> Result<Order, SandboxError> {
        let seed = institution(from).ok_or_else(|| SandboxError::UnknownInstitution(from.into()))?;
        let llave = self.llave(key)?;
        let resolution = self.directory.resolve(&self.registry, &llave).map_err(SandboxError::Directory)?;
        let credencial = self.directory.credencial(&llave).map_err(SandboxError::Directory)?.clone();
        let id = self.next_id(from);
        self.sequence += 1;
        Ok(Order {
            id,
            created: self.now(),
            ordering_red: red_bic(seed.red),
            beneficiary_red: resolution.red,
            beneficiary: resolution.integrante,
            amount,
            debtor: Party {
                name: seed.customer.into(),
                document: format!("00000001{:02}", self.sequence % 100),
                account: format!("{}{:06}", &seed.bic[..4], self.sequence),
            },
            creditor: Party { name: credencial.holder, document: credencial.document, account: credencial.account },
        })
    }

    /// Sends the payment and runs the network until every hop and deadline of it is done.
    pub fn pay(&mut self, from: &str, key: &str, amount: Usd) -> Result<PaymentId, SandboxError> {
        let order = self.order(from, key, amount)?;
        let id = order.id.clone();
        let now = self.now();
        self.net.schedule_payment(now, order);
        self.net.run_until_idle();
        Ok(id)
    }

    pub fn settle(&mut self) -> Settlement {
        let s = self.net.settle().expect("the in-memory ledger never fails");
        self.net.run_until_idle();
        s
    }

    pub fn payment(&self, id: &str) -> Option<(Row, Vec<Hop>)> {
        let row = self.net.rows().iter().find(|r| r.id.as_str() == id)?.clone();
        let hops = self.net.hops(&row.id).into_iter().cloned().collect();
        Some((row, hops))
    }

    pub fn last_payment(&self) -> Option<&Row> {
        self.net.rows().last()
    }

    pub fn institutions(&self) -> Vec<InstitutionView> {
        let positions = self.positions();
        INSTITUTIONS
            .iter()
            .map(|i| {
                let ceiling = self.ceilings[i.id];
                let account = AccountId::Headroom(HeadroomAccount::Integrante(bic(i.bic)));
                let available = pollster::block_on(self.net.ledger().balance(account, AsOf::Latest))
                    .expect("the in-memory ledger never fails")
                    .available();
                InstitutionView {
                    id: i.id,
                    name: i.name,
                    bic: bic(i.bic),
                    red: i.red,
                    sector: i.sector.code(),
                    ceiling,
                    available,
                    light: Light::of(ceiling, available),
                    net: positions.get(&bic(i.bic)).copied().unwrap_or(0),
                }
            })
            .collect()
    }

    pub fn positions(&self) -> BTreeMap<Bic, i128> {
        let entries = pollster::block_on(self.net.ledger().entries(caudal_ledger::EntryQuery::All)).expect("in memory");
        caudal_clearing::malla(&entries).nets
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

pub fn money(cents: i128) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let c = cents.unsigned_abs();
    let whole = (c / 100).to_string();
    let mut grouped = String::new();
    for (i, ch) in whole.chars().enumerate() {
        if i > 0 && (whole.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(ch);
    }
    format!("{sign}{grouped}.{:02}", c % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_seed_has_every_terminal_state_and_a_payment_por_conciliar() {
        let s = Sandbox::new();
        let rows = s.network().rows();
        let has = |state, reason| rows.iter().any(|r| r.state == state && r.reason == reason);
        assert!(has(PaymentState::Acsc, None));
        assert!(has(PaymentState::Acsp, None));
        assert!(has(PaymentState::Rjct, Some(ReasonCode::Ac06)));
        assert!(has(PaymentState::Rjct, Some(ReasonCode::Ac04)));
        assert!(has(PaymentState::Rjct, Some(ReasonCode::Am04)));
        assert!(rows.iter().any(|r| r.por_conciliar));
    }

    #[test]
    fn ceilings_come_from_tabla_19() {
        assert_eq!(ceiling(institution("coop-beta").unwrap()).acceptable, "60000.00".parse().unwrap());
        assert_eq!(ceiling(institution("banco-alfa").unwrap()).acceptable, "4800000.00".parse().unwrap());
        assert_eq!(ceiling(institution("mutual-delta").unwrap()).acceptable, "1920000.00".parse().unwrap());
        assert_eq!(ceiling(institution("fintech-gamma").unwrap()).acceptable, "15000.00".parse().unwrap());
    }

    #[test]
    fn a_ceiling_can_be_overridden_before_the_day_starts() {
        let s = Sandbox::with_ceilings(start_of_day(), SignedDuration::ZERO, &[("coop-beta", Usd::from_cents(100))]);
        let coop = s.institutions().into_iter().find(|i| i.id == "coop-beta").unwrap();
        assert_eq!((coop.ceiling, coop.available), (Usd::from_cents(100), 100));
    }

    #[test]
    fn llaves_resolve_by_catalogue_id_or_value_to_a_masked_name() {
        let s = Sandbox::new();
        let by_id = s.resolve("llave-01").unwrap();
        assert_eq!(by_id.name.as_str(), "Julio C. T. A.");
        assert_eq!(s.resolve("0990000001").unwrap(), by_id);
        assert!(matches!(s.resolve("0991234567"), Err(SandboxError::UnknownLlave(_))));
    }

    #[test]
    fn money_groups_thousands() {
        assert_eq!(money(123_456_789), "1,234,567.89");
        assert_eq!(money(-5), "-0.05");
    }
}
