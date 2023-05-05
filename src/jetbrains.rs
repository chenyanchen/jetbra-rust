use std::path::PathBuf;

use anyhow::{anyhow, Result};

/// Return the path of JetBrains directory.
/// macOS: ~/Library/Application Support/JetBrains
/// TODO confirm: Linux: ~/.config/JetBrains
/// TODO confirm: Windows: %APPDATA%\JetBrains
pub fn path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or(anyhow!("cannot find home directory"))?;
    // TODO: support Linux and Windows
    let jetbrains_dir = home_dir
        .join("Library")
        .join("Application Support")
        .join("JetBrains");
    Ok(jetbrains_dir)
}
