use std::collections::BTreeMap;
use std::fmt;

use caudal_types::Bic;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sector {
    LargeFinancial,
    Financial,
    CooperativeSegment1,
    Mutualista,
    CooperativeSegment2,
    CooperativeSegment3,
    CooperativeSegment4,
    CooperativeSegment5,
    Sedpe,
}

impl Sector {
    pub const fn code(self) -> &'static str {
        match self {
            Self::LargeFinancial => "0001",
            Self::Financial => "0002",
            Self::CooperativeSegment1 => "0003",
            Self::Mutualista => "0004",
            Self::CooperativeSegment2 => "0005",
            Self::CooperativeSegment3 => "0006",
            Self::CooperativeSegment4 => "0007",
            Self::CooperativeSegment5 => "0008",
            Self::Sedpe => "0009",
        }
    }

    /// Encaje in basis points and risk factor in percent. `None` where the catálogo leaves the cells
    /// empty; for a SEDPE only the published base encaje, without its unpublished risk surcharge.
    pub const fn liquidity(self) -> Option<(u32, u32)> {
        match self {
            Self::LargeFinancial => Some((500, 25)),
            Self::Financial => Some((400, 30)),
            Self::CooperativeSegment1 | Self::Mutualista => Some((400, 40)),
            Self::CooperativeSegment2 => Some((250, 100)),
            Self::CooperativeSegment3 => Some((100, 100)),
            Self::CooperativeSegment4 | Self::CooperativeSegment5 => None,
            Self::Sedpe => Some((50, 100)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Active,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Red {
    pub bic: Bic,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integrante {
    pub bic: Bic,
    pub name: String,
    pub sector: Sector,
    pub red: Bic,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    Duplicate(Bic),
    UnknownRed(Bic),
    UnknownIntegrante(Bic),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate(b) => write!(f, "{b} is already registered"),
            Self::UnknownRed(b) => write!(f, "no red {b}"),
            Self::UnknownIntegrante(b) => write!(f, "no integrante {b}"),
        }
    }
}

impl std::error::Error for RegistryError {}

#[derive(Debug, Clone, Default)]
pub struct Registry {
    redes: BTreeMap<Bic, Red>,
    integrantes: BTreeMap<Bic, Integrante>,
}

impl Registry {
    pub fn add_red(&mut self, red: Red) -> Result<(), RegistryError> {
        if self.redes.contains_key(&red.bic) || self.integrantes.contains_key(&red.bic) {
            return Err(RegistryError::Duplicate(red.bic));
        }
        self.redes.insert(red.bic, red);
        Ok(())
    }

    pub fn add_integrante(&mut self, integrante: Integrante) -> Result<(), RegistryError> {
        if self.redes.contains_key(&integrante.bic) || self.integrantes.contains_key(&integrante.bic) {
            return Err(RegistryError::Duplicate(integrante.bic));
        }
        if !self.redes.contains_key(&integrante.red) {
            return Err(RegistryError::UnknownRed(integrante.red));
        }
        self.integrantes.insert(integrante.bic, integrante);
        Ok(())
    }

    pub fn set_status(&mut self, bic: Bic, status: Status) -> Result<(), RegistryError> {
        self.integrantes.get_mut(&bic).map(|i| i.status = status).ok_or(RegistryError::UnknownIntegrante(bic))
    }

    pub fn integrante(&self, bic: Bic) -> Option<&Integrante> {
        self.integrantes.get(&bic)
    }

    pub fn red(&self, bic: Bic) -> Option<&Red> {
        self.redes.get(&bic)
    }

    pub fn redes(&self) -> impl Iterator<Item = &Red> {
        self.redes.values()
    }

    pub fn integrantes(&self) -> impl Iterator<Item = &Integrante> {
        self.integrantes.values()
    }

    pub fn members(&self, red: Bic) -> impl Iterator<Item = &Integrante> {
        self.integrantes.values().filter(move |i| i.red == red)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bic(s: &str) -> Bic {
        s.parse().unwrap()
    }

    fn integrante(b: &str, red: &str) -> Integrante {
        Integrante {
            bic: bic(b),
            name: b.to_lowercase(),
            sector: Sector::CooperativeSegment3,
            red: bic(red),
            status: Status::Active,
        }
    }

    #[test]
    fn an_integrante_belongs_to_a_known_red_and_bics_are_unique() {
        let mut registry = Registry::default();
        assert_eq!(
            registry.add_integrante(integrante("COOPAAEC", "REDAEC")),
            Err(RegistryError::UnknownRed(bic("REDAEC")))
        );
        registry.add_red(Red { bic: bic("REDAEC"), name: "red alfa".into() }).unwrap();
        registry.add_integrante(integrante("COOPAAEC", "REDAEC")).unwrap();
        assert_eq!(
            registry.add_integrante(integrante("COOPAAEC", "REDAEC")),
            Err(RegistryError::Duplicate(bic("COOPAAEC")))
        );
        assert_eq!(
            registry.add_integrante(integrante("REDAEC", "REDAEC")),
            Err(RegistryError::Duplicate(bic("REDAEC")))
        );
        assert_eq!(registry.members(bic("REDAEC")).count(), 1);
    }

    #[test]
    fn suspension_is_recorded() {
        let mut registry = Registry::default();
        registry.add_red(Red { bic: bic("REDAEC"), name: "red alfa".into() }).unwrap();
        registry.add_integrante(integrante("COOPAAEC", "REDAEC")).unwrap();
        registry.set_status(bic("COOPAAEC"), Status::Suspended).unwrap();
        assert_eq!(registry.integrante(bic("COOPAAEC")).unwrap().status, Status::Suspended);
    }
}
