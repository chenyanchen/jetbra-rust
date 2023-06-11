use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::app::App;
use crate::file;

pub struct Uninstaller {
    jetbrains_dir: PathBuf,
    plugins_dir: PathBuf,
    vmoptions_prefixes: Vec<&'static str>,
}

pub struct Args {
    pub remove_dependencies: bool,
    pub apps: Vec<App>,
}

impl Uninstaller {
    pub fn new(jetbrains_dir: PathBuf) -> Self {
        let plugins_dir = jetbrains_dir.join("plugins");
        Self {
            jetbrains_dir,
            plugins_dir,
            vmoptions_prefixes: vec![
                "--add-opens=java.base/jdk.internal.org.objectweb.asm=ALL-UNNAMED",
                "--add-opens=java.base/jdk.internal.org.objectweb.asm.tree=ALL-UNNAMED",
                "-javaagent:",
            ],
        }
    }

    pub fn uninstall(&self, args: &Args) -> Result<()> {
        if args.remove_dependencies {
            self.remove_dependencies()
                .context("Failed to remove dependencies")?;
        }
        args.apps.iter().try_for_each(|app| self.uninstall_app(app))
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

        for path in file::find_directories_with_prefix(&self.jetbrains_dir, &app.concat_name())
            .context("Failed to find dirs by prefix")?
        {
            let vmoptions_filepath = path.join(&vmoptions_filename);
            file::remove_lines_with_prefixes(&vmoptions_filepath, &self.vmoptions_prefixes)
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
