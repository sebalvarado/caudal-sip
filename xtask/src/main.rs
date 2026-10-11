use std::process::ExitCode;

use cargo_metadata::MetadataCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layer {
    Kernel,
    Domain,
    Protocol,
    Adapter,
    App,
}

const DOMAIN: &[(&str, &[&str])] = &[
    ("registry", &[]),
    ("directory", &["registry"]),
    ("ledger", &[]),
    ("fees", &[]),
    ("risk", &["ledger"]),
    ("clearing", &["ledger"]),
    ("payments", &["registry", "directory", "ledger", "fees"]),
    ("reconciliation", &["payments", "ledger", "directory"]),
];

fn layer(name: &str) -> Option<Layer> {
    match name {
        "types" => Some(Layer::Kernel),
        _ if DOMAIN.iter().any(|(d, _)| *d == name) => Some(Layer::Domain),
        "iso20022" | "qr" => Some(Layer::Protocol),
        "msgplane" | "audit" | "north-api" | "south-api" => Some(Layer::Adapter),
        _ if postgres_adapter(name) => Some(Layer::Adapter),
        "sip" | "red" | "gateway" | "caudal" | "tck" => Some(Layer::App),
        _ => None,
    }
}

fn postgres_adapter(name: &str) -> bool {
    name.strip_suffix("-postgres").is_some_and(|d| DOMAIN.iter().any(|(domain, _)| *domain == d) || d == "audit")
}

fn allowed(from: &str, to: &str) -> bool {
    use Layer::*;
    let (Some(f), Some(t)) = (layer(from), layer(to)) else { return false };
    match (f, t) {
        (Kernel, _) => false,
        (_, Kernel) => true,
        (Domain, Domain) => DOMAIN.iter().any(|(d, deps)| *d == from && deps.contains(&to)),
        (Adapter, Domain | Protocol) => from != "msgplane",
        (Adapter, Adapter) => (postgres_adapter(from) && to == "audit") || (from, to) == ("south-api", "msgplane"),
        (App, App) => false,
        (App, _) => true,
        _ => false,
    }
}

fn arch() -> Result<(), Vec<String>> {
    let metadata = MetadataCommand::new().no_deps().exec().map_err(|e| vec![format!("cargo metadata: {e}")])?;
    let mut errors = Vec::new();
    for package in metadata.workspace_packages() {
        let Some(name) = package.name.strip_prefix("caudal-") else {
            if package.name.as_str() != "xtask" {
                errors.push(format!("{}: workspace packages are named caudal-<name>", package.name));
            }
            continue;
        };
        if layer(name).is_none() {
            errors.push(format!("{name}: not in the layer table"));
        }
        let dir = package.manifest_path.parent().expect("a manifest has a directory");
        if !dir.join("AGENTS.md").exists() {
            errors.push(format!("{name}: missing {dir}/AGENTS.md"));
        }
        for dep in &package.dependencies {
            let Some(to) = dep.name.strip_prefix("caudal-") else { continue };
            if to != name && !allowed(name, to) {
                errors.push(format!("{name} -> {to}: edge not allowed"));
            }
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("arch") => match arch() {
            Ok(()) => {
                println!("arch: every crate edge is allowed");
                ExitCode::SUCCESS
            }
            Err(errors) => {
                errors.iter().for_each(|e| eprintln!("arch: {e}"));
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!("usage: cargo xtask arch");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::allowed;

    #[test]
    fn follows_the_adr_tables() {
        assert!(allowed("ledger", "types"));
        assert!(allowed("risk", "ledger"));
        assert!(!allowed("ledger", "risk"));
        assert!(!allowed("types", "ledger"));
        assert!(!allowed("ledger", "ledger-postgres"));
        assert!(!allowed("iso20022", "payments"));
        assert!(allowed("ledger-postgres", "ledger"));
        assert!(allowed("ledger-postgres", "audit"));
        assert!(!allowed("ledger-postgres", "msgplane"));
        assert!(allowed("south-api", "msgplane"));
        assert!(!allowed("msgplane", "payments"));
        assert!(!allowed("sip", "red"));
        assert!(allowed("caudal", "ledger-postgres"));
        assert!(!allowed("ledger", "unknown"));
    }
}
