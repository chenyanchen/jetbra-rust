use anyhow::Result;
use clap::Args;

use crate::application::{find_app, App};

pub struct Installer {
    apps: Vec<App>,
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// Specify applications to install
    #[arg(short, long)]
    app: Option<Vec<String>>,
}

impl Installer {
    pub fn new(apps: Vec<App>) -> Self {
        Self { apps }
    }

    pub fn install(&self, args: &InstallArgs) -> Result<()> {
        Self::install_dependencies()?;
        let pending_apps = match &args.app {
            Some(apps) => {
                let mut pending_apps = Vec::new();
                apps.iter().for_each(|app| match find_app(&self.apps, app) {
                    None => println!("Unknown application {app}"),
                    Some(app) => pending_apps.push(app),
                });
                pending_apps
            }
            None => self.apps.iter().collect(), // install all apps
        };
        pending_apps
            .iter()
            .try_for_each(|app| Self::install_app(app))
    }

    fn install_dependencies() -> Result<()> {
        todo!("install dependencies")
    }

    fn install_app(app: &App) -> Result<()> {
        todo!("install {}", app.name)
    }
}
