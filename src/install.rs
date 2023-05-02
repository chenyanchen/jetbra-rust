use anyhow::Result;
use clap::Args;

use crate::application::App;

pub struct InstallHandler {
    apps: Vec<App>,
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// Specify applications to install
    #[arg(short, long)]
    app: Option<Vec<String>>,
}

impl InstallHandler {
    pub fn new(apps: Vec<App>) -> Self {
        Self { apps }
    }

    pub fn run(&self, args: &InstallArgs) -> Result<()> {
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

fn find_app<'a>(apps: &'a [App], name: &str) -> Option<&'a App> {
    apps.iter()
        .find(|app| app.name == name || app.short == name)
}

pub struct UninstallHandler {
    apps: Vec<App>,
}

#[derive(Args, Debug)]
pub struct UninstallArgs {
    /// Specify applications to uninstall
    #[arg(short, long)]
    app: Option<Vec<String>>,
}

impl UninstallHandler {
    pub fn new(apps: Vec<App>) -> Self {
        Self { apps }
    }

    pub fn run(&self, args: &UninstallArgs) -> Result<()> {
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
