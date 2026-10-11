mod commands;
mod sandbox;
mod scenarios;
mod serve;
mod tui;

use std::io::{BufRead, IsTerminal};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::{Parser, Subcommand};

use crate::commands::{Command, execute, parse};
use crate::sandbox::Sandbox;
use crate::scenarios::Scenario;

/// The whole network in one process: the SIP, two redes and four fictional institutions.
/// With no command, opens the full-screen terminal. All data is fictional.
#[derive(Parser)]
#[command(name = "caudal", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Send a payment from a seeded institution to a llave
    Pay {
        #[arg(long)]
        from: String,
        /// A catalogue id (llave-01) or the llave itself (0990000001)
        #[arg(long)]
        llave: String,
        #[arg(long)]
        amount: String,
        /// Settle the malla afterwards, taking the payment to ACSC
        #[arg(long)]
        settle: bool,
        /// Print every ISO 20022 message
        #[arg(long)]
        xml: bool,
    },
    /// Llave operations
    Llave {
        #[command(subcommand)]
        command: LlaveCmd,
    },
    /// Show a seeded payment
    Payment {
        #[command(subcommand)]
        command: PaymentCmd,
    },
    /// Close and settle the current malla
    Settle,
    /// Headroom, traffic light and net position per institution
    Network,
    /// Run a named load scenario: single, payroll, cutoff or national
    Scenario {
        name: Scenario,
        #[arg(long, default_value_t = 7)]
        seed: u64,
        /// Print the report as JSON
        #[arg(long)]
        json: bool,
    },
    /// Read terminal commands from standard input, one per line, on one network
    Batch,
    /// Check that a file is a message on our ISO 20022 profile
    Validate { file: PathBuf },
    /// Serve the public demo API
    Serve {
        #[arg(long, env = "PORT", default_value_t = 8080)]
        port: u16,
        #[arg(long, env = "CAUDAL_DEMO_ORIGIN", default_value = "https://caudal-sip.pages.dev")]
        origin: String,
        #[arg(long, env = "CAUDAL_DEMO_RESET_SECS", default_value_t = 900)]
        reset_secs: u64,
    },
}

#[derive(Subcommand)]
enum LlaveCmd {
    /// Resolve a llave to its masked name
    Resolve {
        #[arg(long)]
        llave: String,
    },
    /// List the seeded llaves
    List,
}

#[derive(Subcommand)]
enum PaymentCmd {
    /// A payment's hops, states and postings
    Show {
        id: String,
        #[arg(long)]
        xml: bool,
    },
}

fn print(sandbox: &mut Sandbox, command: Command) -> bool {
    let out = execute(sandbox, command);
    print!("{}", out.plain());
    !out.lines.iter().any(|l| l.tone == commands::Tone::Bad && l.text.starts_with("no "))
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        if !std::io::stdout().is_terminal() {
            eprintln!("caudal: the full-screen terminal needs a TTY; try `caudal --help`");
            return ExitCode::FAILURE;
        }
        return match tui::run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("caudal: {e}");
                ExitCode::FAILURE
            }
        };
    };
    let sandbox = Sandbox::new;
    let ok = match command {
        Cmd::Pay { from, llave, amount, settle, xml } => {
            let Ok(amount) = amount.parse() else {
                eprintln!("caudal: amount {amount:?}: digits with at most two decimals");
                return ExitCode::FAILURE;
            };
            let mut s = sandbox();
            let id = match s.pay(&from, &llave, amount) {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("caudal: {e}");
                    return ExitCode::FAILURE;
                }
            };
            let settled = settle.then(|| execute(&mut s, Command::Settle));
            let ok = print(&mut s, Command::Show { id: Some(id.to_string()), xml });
            if let Some(out) = settled {
                print!("\n{}", out.plain());
            }
            ok
        }
        Cmd::Llave { command: LlaveCmd::Resolve { llave } } => print(&mut sandbox(), Command::Resolve(llave)),
        Cmd::Llave { command: LlaveCmd::List } => print(&mut sandbox(), Command::Llaves),
        Cmd::Payment { command: PaymentCmd::Show { id, xml } } => {
            print(&mut sandbox(), Command::Show { id: Some(id), xml })
        }
        Cmd::Settle => print(&mut sandbox(), Command::Settle),
        Cmd::Network => print(&mut sandbox(), Command::Network),
        Cmd::Scenario { name, seed, json } => {
            let report = scenarios::run(name, seed);
            if json {
                println!("{}", serde_json::to_string_pretty(&report).expect("serializable"));
            } else {
                scenarios::render(&report).iter().for_each(|l| println!("{}", l.text));
            }
            true
        }
        Cmd::Batch => {
            let mut s = sandbox();
            let mut ok = true;
            for line in std::io::stdin().lock().lines() {
                let Ok(line) = line else { break };
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                println!("> {line}");
                match parse(line) {
                    Ok(Command::Quit) => break,
                    Ok(command) => ok &= print(&mut s, command),
                    Err(e) => {
                        eprintln!("caudal: {e}");
                        ok = false;
                    }
                }
                println!();
            }
            ok
        }
        Cmd::Validate { file } => match std::fs::read(&file) {
            Err(e) => {
                eprintln!("caudal: {}: {e}", file.display());
                false
            }
            Ok(bytes) => match caudal_iso20022::decode(&bytes) {
                Ok(m) => {
                    println!("valid {} from {} to {}", m.document.definition(), m.header.from, m.header.to);
                    true
                }
                Err(e) => {
                    println!("invalid: {e}");
                    false
                }
            },
        },
        Cmd::Serve { port, origin, reset_secs } => {
            let runtime = tokio::runtime::Runtime::new().expect("a runtime");
            match runtime.block_on(serve::serve(port, &origin, Duration::from_secs(reset_secs.max(60)))) {
                Ok(()) => true,
                Err(e) => {
                    eprintln!("caudal: {e}");
                    false
                }
            }
        }
    };
    if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
