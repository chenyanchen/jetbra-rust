use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::app::{App, Apps};
use crate::file;
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
    pub app: Option<Vec<Apps>>,
}

impl Installer {
    pub fn new(jetbrains_dir: PathBuf) -> Self {
        let netfilter_dir = jetbrains_dir.join("plugins").join("netfilter");
        let netfilter_path = netfilter_dir.join("ja-netfilter.jar");
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
        Archive::new(GzDecoder::new(netfilter_tar_gz.as_ref())).unpack(&self.netfilter_dir)?;
        Ok(())
    }

    fn install_app(&self, app: &App) -> Result<()> {
        // Uninstall app first
        self.uninstaller
            .uninstall_app(app)
            .context("Failed to uninstall")?;

        let vmoptions_filename = format!("{}.vmoptions", app.short);
        let cert_filename = format!("{}.key", app.short);

        for path in file::find_dirs_by_prefix(&self.jetbrains_dir, &app.concat_name())
            .context("Failed to find dirs by prefix")?
        {
            // Append vmoptions
            let vmoptions_filepath = path.join(&vmoptions_filename);
            file::append_lines(vmoptions_filepath, &self.additional_vmoptions)
                .context("Failed to append lines")?;

            // Update certificate file
            let cert_filepath = path.join(&cert_filename);
            fs::write(cert_filepath, Self::build_certificate(&app.code))
                .context("Failed write certificate")?;
        }
        Ok(())
    }

    fn build_certificate(active_code: &String) -> Vec<u8> {
        let header: Vec<u8> = [0xff; 2].to_vec();
        let mut body: Vec<u8> = Vec::from("<certificate-key>\n");
        body.append(&mut active_code.as_bytes().to_vec());
        [header, interleave_byte(&body, 0x00)].concat()
    }
}

/// interleave byte to bytes
/// # Examples
/// ```
/// use jetbra::install::interleave_byte;
/// assert_eq!(interleave_byte(b"Hello", b'a'), b"Haealalaoa");
/// ```
pub fn interleave_byte(bytes: &[u8], byte: u8) -> Vec<u8> {
    bytes.iter().flat_map(|b| vec![*b, byte]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleave_test() {
        assert_eq!(interleave_byte(b"Hello", b'a'), b"Haealalaoa");
    }
}
