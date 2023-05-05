use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;

use crate::app::{App, Apps};
use crate::file;

pub struct Uninstaller {
    jetbrains_dir: PathBuf,
    plugins_dir: PathBuf,
    vmoptions_prefixes: Vec<String>,
}

#[derive(Args)]
pub struct UninstallArgs {
    /// Specify applications to uninstall
    #[arg(short, long, value_enum)]
    pub app: Option<Vec<Apps>>,
}

impl Uninstaller {
    pub fn new(jetbrains_dir: PathBuf) -> Self {
        let plugins_dir = jetbrains_dir.join("plugins");
        Self {
            jetbrains_dir,
            plugins_dir,
            vmoptions_prefixes: vec![
                "--add-opens=java.base/jdk.internal.org.objectweb.asm=ALL-UNNAMED".into(),
                "--add-opens=java.base/jdk.internal.org.objectweb.asm.tree=ALL-UNNAMED".into(),
                "-javaagent:".into(),
            ],
        }
    }

    pub fn uninstall(&self, args: &UninstallArgs) -> Result<()> {
        match &args.app {
            Some(apps) => apps.to_owned(), // uninstall specified apps
            None => {
                self.remove_dependencies()
                    .context("Failed to remove dependencies")?;
                Apps::all() // uninstall all apps
            }
        }
        .iter()
        .try_for_each(|&app| self.uninstall_app(&app.into()))
    }

    fn remove_dependencies(&self) -> Result<()> {
        if self.plugins_dir.exists() {
            fs::remove_dir_all(&self.plugins_dir)?;
        }
        Ok(())
    }

    pub fn uninstall_app(&self, app: &App) -> Result<()> {
        let vmoptions_filename = format!("{}.vmoptions", app.short);
        let cert_filename = format!("{}.key", app.short);

        for path in file::find_dirs_by_prefix(&self.jetbrains_dir, &app.concat_name())
            .context("Failed to find dirs by prefix")?
        {
            let vmoptions_filepath = path.join(&vmoptions_filename);
            file::remove_lines_by_prefixes(&vmoptions_filepath, &self.vmoptions_prefixes)
                .context("Failed to remove lines by prefixes")?;

            let cert_filepath = path.join(&cert_filename);
            if !cert_filepath.exists() {
                continue;
            }
            fs::remove_file(&cert_filepath).context("Failed to remove certificate")?;
        }
        Ok(())
    }
}
