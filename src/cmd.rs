use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};

use crate::app::{App, Apps};
use crate::install::{InstallArgs, Installer};
use crate::jetbrains;
use crate::uninstall::{UninstallArgs, Uninstaller};

#[derive(Default)]
pub struct Jetbra {}

impl Jetbra {
    pub fn run(&self, args: Args) -> Result<()> {
        let mut cmd = Args::command();
        if args.author {
            let author = cmd.get_author().unwrap();
            println!("{}", author);
            return Ok(());
        }
        match &args.command {
            Some(cmd) => self.run_command(cmd)?,
            None => cmd.print_help()?,
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
                .install(args)
                .context("Failed to install")?,
            Commands::Uninstall(args) => Uninstaller::new(jetbrains::path()?)
                .uninstall(args)
                .context("Failed to uninstall")?,
        }
        Ok(())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print author
    #[arg(long)]
    author: bool,
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
