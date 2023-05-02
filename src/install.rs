use anyhow::Result;
use clap::Args;

use crate::application::{App, Apps};

pub struct Installer {}

#[derive(Args)]
pub struct InstallArgs {
    /// Specify applications to install
    #[arg(short, long, value_enum)]
    app: Option<Vec<Apps>>,
}

impl Installer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn install(&self, args: &InstallArgs) -> Result<()> {
        Self::install_dependencies()?;
        match &args.app {
            Some(apps) => apps.to_owned(), // install specified apps
            None => Apps::all(),           // install all apps
        }
        .iter()
        .try_for_each(|&app| Self::install_app(&app.into()))
    }

    fn install_dependencies() -> Result<()> {
        todo!("install dependencies")
    }

    fn install_app(app: &App) -> Result<()> {
        todo!("install {}", app.name)
    }
}
