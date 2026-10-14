use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

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
        let name = match package.name.as_str() {
            "caudal" => Some("caudal"),
            other => other.strip_prefix("caudal-"),
        };
        let Some(name) = name else {
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

const PLAYGROUND: &str = "site/public/simulador/pkg";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("xtask sits in the workspace root").to_path_buf()
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// FNV-1a over every input of the WebAssembly build, so CI can tell a stale artifact without rebuilding it.
fn playground_hash() -> String {
    let root = root();
    let mut files: Vec<PathBuf> =
        ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "apps/caudal/Cargo.toml"].map(|f| root.join(f)).into();
    rust_sources(&root.join("apps/caudal/src"), &mut files);
    for krate in fs::read_dir(root.join("crates")).expect("crates/ exists").flatten() {
        files.push(krate.path().join("Cargo.toml"));
        rust_sources(&krate.path().join("src"), &mut files);
    }
    let mut named: Vec<(String, PathBuf)> = files
        .into_iter()
        .map(|f| (f.strip_prefix(&root).expect("under root").to_string_lossy().replace('\\', "/"), f))
        .collect();
    named.sort();
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for (name, path) in named {
        let bytes = fs::read(&path).unwrap_or_default();
        for b in name.bytes().chain([0]).chain(bytes).chain([0]) {
            hash = (hash ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    }
    format!("{hash:016x}")
}

fn run(program: &str, args: &[&str]) -> Result<(), String> {
    let status =
        Command::new(program).args(args).current_dir(root()).status().map_err(|e| format!("{program}: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("{program} {} failed", args.join(" "))) }
}

fn playground(check: bool) -> Result<String, String> {
    let stamp = root().join(PLAYGROUND).join("SOURCE");
    let hash = playground_hash();
    if check {
        let built = fs::read_to_string(&stamp).unwrap_or_default();
        return if built.trim() == hash {
            Ok(format!("playground: {PLAYGROUND} is built from the current sources"))
        } else {
            Err(format!("playground: {PLAYGROUND} is stale; run `just playground` and commit it"))
        };
    }
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let target = ["--target", "wasm32-unknown-unknown"];
    run(
        &cargo,
        &[&["rustc", "-p", "caudal", "--lib", "--profile", "wasm", "--crate-type", "cdylib"][..], &target].concat(),
    )?;
    let wasm = "target/wasm32-unknown-unknown/wasm/caudal.wasm";
    run("wasm-bindgen", &["--target", "web", "--no-typescript", "--out-dir", PLAYGROUND, wasm])?;
    fs::write(&stamp, format!("{hash}\n")).map_err(|e| format!("{}: {e}", stamp.display()))?;
    Ok(format!("playground: built {PLAYGROUND} ({hash})"))
}

fn report(result: Result<String, String>) -> ExitCode {
    match result {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("playground") {
        return report(playground(args.iter().any(|a| a == "--check")));
    }
    match args.first().map(String::as_str) {
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
            eprintln!("usage: cargo xtask arch | cargo xtask playground [--check]");
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
