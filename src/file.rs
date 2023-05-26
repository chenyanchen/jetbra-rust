use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

/// Removes all lines from a file that start with any of the given prefixes.
pub fn remove_lines_with_prefixes(path: &Path, prefixes: &[String]) -> Result<()> {
    // If the file doesn't exist, there's nothing to do.
    if !path.exists() {
        return Ok(());
    }

    // Create a new file with a unique name based on the current timestamp.
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let new_path = path.with_extension(format!("{}.{}", timestamp, "new"));
    let new_file = File::create(&new_path)?;

    // Open the original file for reading.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Open the new file for writing.
    let mut writer = BufWriter::new(new_file);

    // Iterate over each line in the file.
    for line in reader.lines() {
        let line = line?;

        // If the line doesn't start with any of the given prefixes, write it to the new file.
        if !prefixes.iter().any(|prefix| line.starts_with(prefix)) {
            writeln!(writer, "{}", line)?;
        }
    }

    // Replace the original file with the new file.
    fs::rename(&new_path, path)?;

    Ok(())
}

/// Finds all directories in the given directory that start with the given prefix.
pub fn find_directories_with_prefix(dir: &Path, prefix: &str) -> Result<Vec<PathBuf>> {
    // If the directory doesn't exist, there are no directories to find.
    if !dir.exists() {
        return Ok(vec![]);
    }

    // Use `read_dir` to lazily read the contents of the directory and filter the results.
    let dirs = fs::read_dir(dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir()
                && path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()?
                    .starts_with(prefix)
            {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    Ok(dirs)
}

/// Appends the given lines to the end of the file at the given path.
pub fn append_lines_to_file<P: AsRef<Path>>(path: P, lines: &[String]) -> Result<()> {
    // Open the file in append mode, creating it if it doesn't exist.
    let file = OpenOptions::new().append(true).create(true).open(path)?;
    let mut writer = BufWriter::new(file);

    // Write each line to the file.
    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}
