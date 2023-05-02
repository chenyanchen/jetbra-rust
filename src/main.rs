use anyhow::Result;
use clap::{Parser, Subcommand};

use application::App;
use install::{InstallArgs, Installer};
use uninstall::{UninstallArgs, Uninstaller};

mod application;
mod install;
mod uninstall;

fn main() -> Result<()> {
    let args = Args::parse();
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
    Jetbra::new(args, apps).run()
}

struct Jetbra {
    args: Args,
    apps: Vec<App>,
}

impl Jetbra {
    fn new(args: Args, apps: Vec<App>) -> Self {
        Self { args, apps }
    }

    fn run(&self) -> Result<()> {
        match &self.args.command {
            Some(cmd) => match cmd {
                Command::List => self.apps.iter().for_each(|app| {
                    println!("{} ({})", app.name, app.short);
                }),
                Command::Install(args) => Installer::new(self.apps.clone()).install(args)?,
                Command::Uninstall(args) => Uninstaller::new(self.apps.clone()).uninstall(args)?,
            },
            None => println!("Use --help to see the usage"),
        }
        Ok(())
    }
}

#[derive(Parser)]
#[command(
    name = "jetbra",
    author = "Yanchen Chen <yanchen1610@gmail.com>",
    version = "0.1.3",
    about = "Filter network for Java programs",
    long_about = None,
)]
struct Args {
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
