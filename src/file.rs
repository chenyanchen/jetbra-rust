use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;

pub fn remove_lines_by_prefixes(path: &Path, prefixes: &[String]) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let data = fs::read_to_string(path)?;
    let mut buffer: Vec<u8> = Vec::with_capacity(data.len());
    for line in data.lines() {
        if !prefixes.iter().any(|prefix| line.starts_with(prefix)) {
            writeln!(buffer, "{}", line)?;
        }
    }
    fs::write(path, buffer)?;
    Ok(())
}

pub fn find_dirs_by_prefix(dir: &Path, prefix: &str) -> Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let dirs = fs::read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_dir() {
                return None;
            }
            if entry.file_name().to_str()?.starts_with(prefix) {
                return Some(path);
            }
            None
        })
        .collect();
    Ok(dirs)
}

pub fn append_lines<P: AsRef<Path>>(path: P, lines: &[String]) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    lines
        .iter()
        .try_for_each(|line| writeln!(file, "{}", line))?;
    Ok(())
}
