use std::collections::BTreeMap;
use std::fmt;

use caudal_registry::{Registry, Status};
use caudal_types::{Bic, InvalidValue, LlaveType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Llave {
    kind: LlaveType,
    value: String,
}

impl Llave {
    pub fn new(kind: LlaveType, value: &str) -> Result<Self, InvalidValue> {
        let alnum = |s: &str| s.bytes().all(|b| b.is_ascii_alphanumeric());
        let valid = match kind {
            LlaveType::Celular => value.len() == 10 && alnum(value),
            LlaveType::Identificacion => (10..=13).contains(&value.len()) && value.bytes().all(|b| b.is_ascii_digit()),
            LlaveType::Correo => value.len() <= 250 && valid_email(value),
            LlaveType::Alfanumerico => (1..=20).contains(&value.len()) && alnum(value),
        };
        match valid {
            true => Ok(Self { kind, value: value.to_owned() }),
            false => Err(InvalidValue::new("Llave", "a value in the format of its type")),
        }
    }

    pub fn kind(&self) -> LlaveType {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn valid_email(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else { return false };
    let labels: Vec<&str> = domain.split('.').collect();
    !local.is_empty()
        && !local.contains(char::is_whitespace)
        && labels.len() >= 2
        && labels.iter().all(|l| !l.is_empty() && l.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-'))
}

impl fmt::Display for Llave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.value)
    }
}

/// The only way to show a holder's name outside its red: the first given name and the initial of every
/// other name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskedName(String);

impl MaskedName {
    pub fn mask(full_name: &str) -> Self {
        let mut words = full_name.split_whitespace();
        let first = words.next().unwrap_or_default().to_owned();
        let initials = words.filter_map(|w| w.chars().next()).map(|c| format!("{}.", c.to_uppercase()));
        Self(std::iter::once(first).chain(initials).collect::<Vec<_>>().join(" "))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MaskedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
    Corriente,
    Ahorros,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credencial {
    pub holder: String,
    pub document: String,
    pub account: String,
    pub account_type: AccountType,
    pub integrante: Bic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    pub llave: Llave,
    pub name: MaskedName,
    pub integrante: Bic,
    pub red: Bic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectoryError {
    Taken,
    NotFound,
    UnknownIntegrante(Bic),
    Inactive(Bic),
}

impl fmt::Display for DirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Taken => f.write_str("the llave is already registered nationally"),
            Self::NotFound => f.write_str("no such llave"),
            Self::UnknownIntegrante(b) => write!(f, "no integrante {b}"),
            Self::Inactive(b) => write!(f, "integrante {b} is suspended"),
        }
    }
}

impl std::error::Error for DirectoryError {}

#[derive(Debug, Clone, Default)]
pub struct Directory {
    central: BTreeMap<Llave, Bic>,
    distributed: BTreeMap<Bic, BTreeMap<Llave, Credencial>>,
}

impl Directory {
    pub fn register(
        &mut self,
        registry: &Registry,
        llave: Llave,
        credencial: Credencial,
    ) -> Result<(), DirectoryError> {
        let owner = registry
            .integrante(credencial.integrante)
            .ok_or(DirectoryError::UnknownIntegrante(credencial.integrante))?;
        if self.central.contains_key(&llave) {
            return Err(DirectoryError::Taken);
        }
        self.central.insert(llave.clone(), owner.red);
        self.distributed.entry(owner.red).or_default().insert(llave, credencial);
        Ok(())
    }

    pub fn resolve(&self, registry: &Registry, llave: &Llave) -> Result<Resolution, DirectoryError> {
        let credencial = self.credencial(llave)?;
        let integrante = registry
            .integrante(credencial.integrante)
            .ok_or(DirectoryError::UnknownIntegrante(credencial.integrante))?;
        if integrante.status == Status::Suspended {
            return Err(DirectoryError::Inactive(integrante.bic));
        }
        Ok(Resolution {
            llave: llave.clone(),
            name: MaskedName::mask(&credencial.holder),
            integrante: integrante.bic,
            red: integrante.red,
        })
    }

    /// Read by the beneficiary red only, to credit the account.
    pub fn credencial(&self, llave: &Llave) -> Result<&Credencial, DirectoryError> {
        let red = self.central.get(llave).ok_or(DirectoryError::NotFound)?;
        self.distributed.get(red).and_then(|d| d.get(llave)).ok_or(DirectoryError::NotFound)
    }

    pub fn llaves(&self) -> impl Iterator<Item = &Llave> {
        self.central.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use caudal_registry::{Integrante, Red, Sector};

    fn bic(s: &str) -> Bic {
        s.parse().unwrap()
    }

    fn registry() -> Registry {
        let mut r = Registry::default();
        r.add_red(Red { bic: bic("REDBEC"), name: "red beta".into() }).unwrap();
        r.add_integrante(Integrante {
            bic: bic("COOPBBEC"),
            name: "coop-beta".into(),
            sector: Sector::CooperativeSegment3,
            red: bic("REDBEC"),
            status: Status::Active,
        })
        .unwrap();
        r
    }

    fn credencial() -> Credencial {
        Credencial {
            holder: "julio césar tienda andrade".into(),
            document: "0000000001".into(),
            account: "4400000001".into(),
            account_type: AccountType::Ahorros,
            integrante: bic("COOPBBEC"),
        }
    }

    #[test]
    fn llave_formats_follow_their_type() {
        assert!(Llave::new(LlaveType::Celular, "0990000001").is_ok());
        assert!(Llave::new(LlaveType::Celular, "099000001").is_err());
        assert!(Llave::new(LlaveType::Identificacion, "0000000001001").is_ok());
        assert!(Llave::new(LlaveType::Identificacion, "A000000001").is_err());
        assert!(Llave::new(LlaveType::Correo, "tienda@ejemplo.ec").is_ok());
        for bad in ["tienda", "tienda@ejemplo", "@ejemplo.ec", "tienda@ejem plo.ec"] {
            assert!(Llave::new(LlaveType::Correo, bad).is_err(), "{bad}");
        }
        assert!(Llave::new(LlaveType::Alfanumerico, "TIENDAJULIO").is_ok());
        assert!(Llave::new(LlaveType::Alfanumerico, "UNA-LLAVE").is_err());
        assert!(Llave::new(LlaveType::Alfanumerico, &"A".repeat(21)).is_err());
    }

    #[test]
    fn names_are_masked_after_the_first() {
        assert_eq!(MaskedName::mask("julio césar tienda andrade").as_str(), "julio C. T. A.");
        assert_eq!(MaskedName::mask("María").as_str(), "María");
        assert_eq!(MaskedName::mask("  ").as_str(), "");
    }

    #[test]
    fn a_llave_is_unique_nationally_and_resolves_to_a_masked_name() {
        let registry = registry();
        let mut directory = Directory::default();
        let llave = Llave::new(LlaveType::Celular, "0990000001").unwrap();
        directory.register(&registry, llave.clone(), credencial()).unwrap();
        assert_eq!(directory.register(&registry, llave.clone(), credencial()), Err(DirectoryError::Taken));
        let resolution = directory.resolve(&registry, &llave).unwrap();
        assert_eq!(resolution.name.as_str(), "julio C. T. A.");
        assert_eq!((resolution.integrante, resolution.red), (bic("COOPBBEC"), bic("REDBEC")));
        let unknown = Llave::new(LlaveType::Celular, "0990000002").unwrap();
        assert_eq!(directory.resolve(&registry, &unknown), Err(DirectoryError::NotFound));
    }

    #[test]
    fn a_suspended_integrante_cannot_be_paid_by_llave() {
        let mut registry = registry();
        let mut directory = Directory::default();
        let llave = Llave::new(LlaveType::Celular, "0990000001").unwrap();
        directory.register(&registry, llave.clone(), credencial()).unwrap();
        registry.set_status(bic("COOPBBEC"), Status::Suspended).unwrap();
        assert_eq!(directory.resolve(&registry, &llave), Err(DirectoryError::Inactive(bic("COOPBBEC"))));
    }
}
