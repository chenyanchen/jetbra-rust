use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::application::{App, Apps};
use crate::install::find_dirs_by_prefix;

pub struct Uninstaller {
    jetbrains_dir: PathBuf,
    plugins_dir: PathBuf,
    vmoptions_prefixes: Vec<String>,
}

#[derive(Args)]
pub struct UninstallArgs {
    /// Specify applications to uninstall
    #[arg(short, long, value_enum)]
    app: Option<Vec<Apps>>,
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
                self.remove_dependencies()?; // remove dependencies
                Apps::all() // uninstall all apps
            }
        }
        .iter()
        .try_for_each(|&app| self.uninstall_app(&app.into()))
    }

    fn remove_dependencies(&self) -> Result<()> {
        fs::remove_dir_all(&self.plugins_dir)?;
        Ok(())
    }

    pub fn uninstall_app(&self, app: &App) -> Result<()> {
        let mut vmoptions_filename = app.short.clone();
        vmoptions_filename.push_str(".vmoptions");

        for path in find_dirs_by_prefix(&self.jetbrains_dir, app.name.replace(' ', "").as_str())? {
            let vmoptions_path = path.join(vmoptions_filename.as_str());
            self.remove_vmoptions(&vmoptions_path)?;
            println!("uninstall app from: {:?}", vmoptions_path);
        }
        Ok(())
    }

    fn remove_vmoptions(&self, vmoptions_path: &PathBuf) -> Result<()> {
        let vmoptions = fs::read_to_string(vmoptions_path)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(vmoptions.len());
        for line in vmoptions.lines() {
            if !self
                .vmoptions_prefixes
                .iter()
                .any(|prefix| line.starts_with(prefix))
            {
                writeln!(buffer, "{}", line)?;
            }
        }
        fs::write(vmoptions_path, buffer)?;
        Ok(())
    }
}
