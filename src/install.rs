use std::fs;
use std::fs::read_dir;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::application::{App, Apps};
use crate::uninstall::Uninstaller;

pub struct Installer {
    jetbrains_dir: PathBuf,
    netfilter_dir: PathBuf,

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
        let netfilter_dir = jetbrains_dir.join("plugins").join("netfilter");
        let netfilter_path = netfilter_dir.join("ja-netfilter.jar").as_path().to_owned();
        Self {
            jetbrains_dir: jetbrains_dir.clone(),
            netfilter_dir,
            additional_vmoptions: vec![
                "--add-opens=java.base/jdk.internal.org.objectweb.asm=ALL-UNNAMED".into(),
                "--add-opens=java.base/jdk.internal.org.objectweb.asm.tree=ALL-UNNAMED".into(),
                format!("-javaagent:{}=jetbrains", netfilter_path.to_str().unwrap()),
            ],
            uninstaller: Uninstaller::new(jetbrains_dir),
        }
    }

    pub fn install(&self, args: &InstallArgs) -> Result<()> {
        self.install_dependencies()
            .context("Failed to install dependencies")?; // install dependencies
        match &args.app {
            Some(apps) => apps.to_owned(), // install specified apps
            None => Apps::all(),           // install all apps
        }
        .iter()
        .try_for_each(|&app| self.install_app(&app.into()))
    }

    fn install_dependencies(&self) -> Result<()> {
        // Unpack netfilter.tar.gz to $(JetBrains)/plugins/netfilter
        let netfilter_tar_gz = include_bytes!("netfilter.tar.gz");
        let gz = GzDecoder::new(netfilter_tar_gz.as_ref());
        Archive::new(gz).unpack(&self.netfilter_dir)?;
        Ok(())
    }

    fn install_app(&self, app: &App) -> Result<()> {
        // Uninstall app first
        self.uninstaller
            .uninstall_app(app)
            .context("Failed to uninstall")?;

        let vmoptions_filename = format!("{}.vmoptions", app.short);
        let cert_filename = format!("{}.key", app.short);

        for path in find_dirs_by_prefix(&self.jetbrains_dir, app.name.replace(' ', "").as_str())
            .context("Failed to find dirs by prefix")?
        {
            // Append vmoptions
            self.append_lines(
                path.join(&vmoptions_filename).as_path(),
                &self.additional_vmoptions,
            )?;
            // Update certificate file
            let certificate = Self::build_certificate(app.code.clone());
            fs::write(path.join(&cert_filename), certificate)
                .context("Failed write certificate")?;
        }
        Ok(())
    }

    fn append_lines(&self, path: &Path, lines: &[String]) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        lines
            .iter()
            .try_for_each(|line| writeln!(file, "{}", line))?;
        Ok(())
    }

    fn build_certificate(active_code: String) -> Vec<u8> {
        let header: Vec<u8> = [0xff; 2].to_vec();
        let mut body: Vec<u8> = Vec::from("<certificate-key>\n");
        body.append(&mut active_code.as_bytes().to_vec());
        [header, interleave_byte(&body, 0x00)].concat()
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

/// interleave byte to bytes
/// ```
/// interleave_byte("Hello".as_bytes(), 'a') -> "Haealalaoa"
/// ```
fn interleave_byte(bytes: &[u8], byte: u8) -> Vec<u8> {
    bytes.iter().flat_map(|b| vec![*b, byte]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleave_test() {
        assert_eq!(
            interleave_byte("Hello".as_bytes(), b'a'),
            "Haealalaoa".as_bytes()
        );
    }
}
