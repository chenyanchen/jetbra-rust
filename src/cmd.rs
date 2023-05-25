use anyhow::{Context, Result};
use clap::{Args, CommandFactory, Parser, Subcommand};

use crate::app::{App, Apps};
use crate::install::Installer;
use crate::uninstall::Uninstaller;
use crate::{install, jetbrains, uninstall};

#[derive(Default)]
pub struct Jetbra {}

impl Jetbra {
    pub fn run(&self, args: JetbraArgs) -> Result<()> {
        match &args.command {
            Some(cmd) => self.run_command(cmd)?,
            None => JetbraArgs::command().print_help()?,
        }
        Ok(())
    }

    fn run_command(&self, cmd: &Commands) -> Result<()> {
        match cmd {
            Commands::List => Apps::all().iter().for_each(|&app| {
                let app: App = app.into();
                println!("{} ({})", app.name, app.short);
            }),
            Commands::Install(args) => Installer::new(jetbrains::path()?)
                .install(&args.into())
                .context("Failed to install")?,
            Commands::Uninstall(args) => Uninstaller::new(jetbrains::path()?)
                .uninstall(&args.into())
                .context("Failed to uninstall")?,
        }
        Ok(())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct JetbraArgs {
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
    Uninstall(InstallArgs),
}

#[derive(Args)]
struct InstallArgs {
    /// Specify apps, all by default
    #[arg(short, long, value_enum)]
    app: Option<Vec<Apps>>,
}

impl From<&InstallArgs> for install::Args {
    fn from(args: &InstallArgs) -> Self {
        match &args.app {
            None => install::Args {
                apps: Apps::all().iter().map(|&app| app.into()).collect(),
            },
            Some(apps) => install::Args {
                apps: apps.iter().map(|&app| app.into()).collect(),
            },
        }
    }
}

impl From<&InstallArgs> for uninstall::Args {
    fn from(args: &InstallArgs) -> Self {
        match &args.app {
            None => uninstall::Args {
                remove_dependencies: true,
                apps: Apps::all().iter().map(|&app| app.into()).collect(),
            },
            Some(apps) => uninstall::Args {
                remove_dependencies: false,
                apps: apps.iter().map(|&app| app.into()).collect(),
            },
        }
    }
}
