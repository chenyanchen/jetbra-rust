use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::application::{App, Apps};
use crate::install::{InstallArgs, Installer};
use crate::uninstall::{UninstallArgs, Uninstaller};

pub struct Jetbra {}

impl Jetbra {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, args: Args) -> Result<()> {
        match &args.command {
            Some(cmd) => match cmd {
                Command::List => Apps::all().iter().for_each(|&app| {
                    let app: App = app.into();
                    println!("{} ({})", app.name, app.short);
                }),
                Command::Install(args) => Installer::new().install(args)?,
                Command::Uninstall(args) => Uninstaller::new().uninstall(args)?,
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
pub struct Args {
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
