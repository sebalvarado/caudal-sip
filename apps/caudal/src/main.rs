mod tui;

use std::io::{IsTerminal, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use caudal::commands::{self, Command, execute};
use caudal::sandbox::Sandbox;
use caudal::scenarios::Scenario;
use caudal::script;
use clap::{Parser, Subcommand};

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
    /// Run a built-in scenario: single, payroll, cutoff or national
    Scenario {
        name: Scenario,
        /// Replace the scenario's seed
        #[arg(long)]
        seed: Option<u64>,
        /// Print the report as JSON
        #[arg(long)]
        json: bool,
        /// Print the scenario's script instead of running it
        #[arg(long)]
        source: bool,
    },
    /// Run a scenario script from a file, or `-` for standard input
    Run {
        file: PathBuf,
        /// Replace the script's seed
        #[arg(long)]
        seed: Option<u64>,
        /// Print the log, the report and the payments as JSON
        #[arg(long)]
        json: bool,
    },
    /// Check that a file is a message on our ISO 20022 profile
    Validate { file: PathBuf },
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

fn run_script(source: &str, seed: Option<u64>, json: bool) -> bool {
    match script::run(source, seed) {
        Err(e) => {
            eprintln!("caudal: {e}");
            false
        }
        Ok(run) if json => {
            let out = serde_json::json!({ "report": run.report, "log": run.log, "payments": run.payments });
            println!("{}", serde_json::to_string_pretty(&out).expect("serializable"));
            true
        }
        Ok(run) => {
            for l in &run.log {
                println!("{}  {}", l.at, l.text);
            }
            if !run.log.is_empty() {
                println!();
            }
            script::render(&run.report).iter().for_each(|l| println!("{}", l.text));
            true
        }
    }
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
        Cmd::Scenario { name, source: true, .. } => {
            print!("{}", name.source());
            true
        }
        Cmd::Scenario { name, seed, json, .. } => run_script(name.source(), seed, json),
        Cmd::Run { file, seed, json } => {
            let mut source = String::new();
            let read = if file.as_os_str() == "-" {
                std::io::stdin().read_to_string(&mut source).map(|_| ())
            } else {
                std::fs::read_to_string(&file).map(|s| source = s)
            };
            if let Err(e) = read {
                eprintln!("caudal: {}: {e}", file.display());
                return ExitCode::FAILURE;
            }
            run_script(&source, seed, json)
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
    };
    if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
