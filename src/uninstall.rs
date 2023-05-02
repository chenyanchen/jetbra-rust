use anyhow::Result;
use clap::Args;

use crate::application::{find_app, App};

pub struct Uninstaller {
    apps: Vec<App>,
}

#[derive(Args, Debug)]
pub struct UninstallArgs {
    /// Specify applications to uninstall
    #[arg(short, long)]
    app: Option<Vec<String>>,
}

impl Uninstaller {
    pub fn new(apps: Vec<App>) -> Self {
        Self { apps }
    }

    pub fn uninstall(&self, args: &UninstallArgs) -> Result<()> {
        match &args.app {
            Some(apps) => apps
                .iter()
                .try_for_each(|app| match find_app(&self.apps, app) {
                    None => {
                        println!("Unknown application {app}");
                        Ok(())
                    }
                    Some(app) => Self::uninstall_app(app),
                }),
            None => {
                // uninstall all apps and dependencies
                self.apps.iter().try_for_each(Self::uninstall_app)?;
                Self::remove_dependencies()
            }
        }
    }

    fn uninstall_app(app: &App) -> Result<()> {
        todo!("uninstall {}", app.name)
    }

    fn remove_dependencies() -> Result<()> {
        todo!("remove dependencies")
    }
}
