use std::path::PathBuf;

use anyhow::{anyhow, Result};
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
                Command::Install(args) => Installer::new(Self::jetbrains_dir()?).install(args)?,
                Command::Uninstall(args) => {
                    Uninstaller::new(Self::jetbrains_dir()?).uninstall(args)?
                }
            },
            None => println!("Use --help to see the usage"),
        }
        Ok(())
    }

    pub fn jetbrains_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or(anyhow!("cannot find home directory"))?;
        // TODO: support Linux and Windows
        let jetbrains_dir = home_dir
            .join("Library")
            .join("Application Support")
            .join("JetBrains");
        Ok(jetbrains_dir)
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
    /// List all supported applications
    List,
    /// Install jetbra for current user
    Install(InstallArgs),
    /// Uninstall jetbra for current user
    Uninstall(UninstallArgs),
}
