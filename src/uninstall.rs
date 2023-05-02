use anyhow::Result;
use clap::Args;

use crate::application::{App, Apps};

pub struct Uninstaller {}

#[derive(Args)]
pub struct UninstallArgs {
    /// Specify applications to uninstall
    #[arg(short, long, value_enum)]
    app: Option<Vec<Apps>>,
}

impl Uninstaller {
    pub fn new() -> Self {
        Self {}
    }

    pub fn uninstall(&self, args: &UninstallArgs) -> Result<()> {
        match &args.app {
            Some(apps) => apps.to_owned(), // uninstall specified apps
            None => {
                Self::remove_dependencies()?; // remove dependencies
                Apps::all() // uninstall all apps
            }
        }
        .iter()
        .try_for_each(|&app| Self::uninstall_app(&app.into()))
    }

    fn remove_dependencies() -> Result<()> {
        todo!("remove dependencies")
    }

    fn uninstall_app(app: &App) -> Result<()> {
        todo!("uninstall {}", app.name)
    }
}
