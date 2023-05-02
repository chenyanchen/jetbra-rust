use std::fs;
use std::fs::read_dir;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Args;

use crate::application::{App, Apps};
use crate::uninstall::Uninstaller;

pub struct Installer {
    jetbrains_dir: PathBuf,
    additional_vmoptions: Vec<String>,

    uninstaller: Uninstaller,
}

#[derive(Args)]
pub struct InstallArgs {
    /// Specify applications to install
    #[arg(short, long, value_enum)]
    app: Option<Vec<Apps>>,
}

impl Installer {
    pub fn new(jetbrains_dir: PathBuf) -> Self {
        let agent_path = jetbrains_dir
            .join("plugins")
            .join("netfilter")
            .join("ja-netfilter.jar")
            .as_path()
            .to_owned();
        Self {
            jetbrains_dir: jetbrains_dir.clone(),
            additional_vmoptions: vec![
                "--add-opens=java.base/jdk.internal.org.objectweb.asm=ALL-UNNAMED".into(),
                "--add-opens=java.base/jdk.internal.org.objectweb.asm.tree=ALL-UNNAMED".into(),
                format!("-javaagent:{}=jetbrains", agent_path.to_str().unwrap()),
            ],
            uninstaller: Uninstaller::new(jetbrains_dir),
        }
    }

    pub fn install(&self, args: &InstallArgs) -> Result<()> {
        Self::install_dependencies()?;
        match &args.app {
            Some(apps) => apps.to_owned(), // install specified apps
            None => Apps::all(),           // install all apps
        }
        .iter()
        .try_for_each(|&app| self.install_app(&app.into()))
    }

    fn install_dependencies() -> Result<()> {
        println!("install dependencies");
        Ok(())
    }

    fn install_app(&self, app: &App) -> Result<()> {
        // Uninstall app first
        self.uninstaller.uninstall_app(app)?;

        let mut vmoptions_filename = app.short.clone();
        vmoptions_filename.push_str(".vmoptions");

        for path in find_dirs_by_prefix(&self.jetbrains_dir, app.name.replace(' ', "").as_str())? {
            let vmoptions_path = path.join(vmoptions_filename.as_str());
            self.append_vmoptions(&vmoptions_path)?;
            println!("install app at: {:?}", vmoptions_path);
        }
        Ok(())
    }

    fn append_vmoptions(&self, path: &Path) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        self.additional_vmoptions
            .iter()
            .try_for_each(|line| writeln!(file, "{}", line))?;
        Ok(())
    }
}

pub fn find_dirs_by_prefix(dir: &Path, prefix: &str) -> Result<Vec<PathBuf>> {
    let dirs = read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_dir() {
                return None;
            }
            let file_name = entry.file_name();
            if !file_name.to_str()?.starts_with(prefix) {
                return None;
            }
            Some(path)
        })
        .collect();
    Ok(dirs)
}
