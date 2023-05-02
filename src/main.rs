use anyhow::Result;
use clap::{Parser, Subcommand};

use application::App;
use install::{InstallArgs, InstallHandler, UninstallArgs, UninstallHandler};

mod application;
mod install;

#[derive(Parser)]
#[command(
    name = "jetbra",
    author = "Yanchen Chen <yanchen1610@gmail.com>",
    version = "0.1.3",
    about = "Filter network for Java programs",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// List all available applications
    List,
    /// Install applications
    Install(InstallArgs),
    /// Uninstall applications
    Uninstall(UninstallArgs),
}

fn main() -> Result<()> {
    let jetbra_args = Cli::parse();
    let apps = vec![
        App {
            name: "IntelliJ IDEA".to_string(),
            short: "idea".to_string(),
            code: "IIU".to_string(),
        },
        App {
            name: "PyCharm".to_string(),
            short: "pycharm".to_string(),
            code: "PCP".to_string(),
        },
    ];
    let jetbra_handler = JetbraHandler::new(jetbra_args, apps);
    jetbra_handler.run()
}

struct JetbraHandler {
    args: Cli,
    apps: Vec<App>,
}

impl JetbraHandler {
    fn new(args: Cli, apps: Vec<App>) -> Self {
        Self { args, apps }
    }

    fn run(&self) -> Result<()> {
        match &self.args.command {
            Some(cmd) => match cmd {
                Command::List => self.apps.iter().for_each(|app| {
                    println!("{} ({})", app.name, app.short);
                }),
                Command::Install(args) => InstallHandler::new(self.apps.clone()).run(args)?,
                Command::Uninstall(args) => UninstallHandler::new(self.apps.clone()).run(args)?,
            },
            None => println!("Use --help to see the usage"),
        }
        Ok(())
    }
}
