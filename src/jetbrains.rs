use std::path::PathBuf;

use anyhow::{anyhow, Result};

/// Return the path of JetBrains directory.
/// macOS: ~/Library/Application Support/JetBrains
#[cfg(target_os = "macos")]
pub fn path() -> Result<PathBuf> {
    let home_dir = dirs::config_dir().ok_or(anyhow!("cannot find home directory"))?;
    let jetbrains_dir = home_dir.join("JetBrains");
    Ok(jetbrains_dir)
}

// TODO: Make sure it's correct.
#[cfg(target_os = "linux")]
pub fn path() -> Result<PathBuf> {
    let home_dir = dirs::config_dir().ok_or(anyhow!("cannot find home directory"))?;
    let jetbrains_dir = home_dir.join("JetBrains");
    Ok(jetbrains_dir)
}

// TODO: Make sure the right path of JetBrains directory on Windows.
#[cfg(target_os = "windows")]
pub fn path() -> Result<PathBuf> {
    let home_dir = dirs::config_dir().ok_or(anyhow!("cannot find home directory"))?;
    let jetbrains_dir = home_dir.join("JetBrains");
    Ok(jetbrains_dir)
}
