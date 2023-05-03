use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{CommandFactory, Parser, Subcommand};

use crate::application::{App, Apps};
use crate::install::{InstallArgs, Installer};
use crate::uninstall::{UninstallArgs, Uninstaller};

#[derive(Default)]
pub struct Jetbra {}

impl Jetbra {
    pub fn run(&self, args: Args) -> Result<()> {
        match &args.command {
            Some(cmd) => self.run_command(cmd)?,
            None => Args::command().print_help()?,
        }
        Ok(())
    }

    fn run_command(&self, cmd: &Commands) -> Result<()> {
        match cmd {
            Commands::List => Apps::all().iter().for_each(|&app| {
                let app: App = app.into();
                println!("{} ({})", app.name, app.short);
            }),
            Commands::Install(args) => Installer::new(Self::jetbrains_dir()?)
                .install(args)
                .context("Failed to install")?,
            Commands::Uninstall(args) => Uninstaller::new(Self::jetbrains_dir()?)
                .uninstall(args)
                .context("Failed to uninstall")?,
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
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all supported applications
    List,
    /// Install jetbra for current user
    Install(InstallArgs),
    /// Uninstall jetbra for current user
    Uninstall(UninstallArgs),
}
