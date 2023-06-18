use std::process::{Command, Stdio};

use anyhow::Result;

/// exec("wmic bios get serialnumber")
#[cfg(target_os = "windows")]
fn serial_number() -> Result<String> {
    let wmic = Command::new("wmic")
        .args(["bios", "get", "serialnumber"])
        .output()?;
    Ok(String::from_utf8(wmic.stdout)?)
}

/// exec("dmidecode -s system-serial-number")
#[cfg(target_os = "linux")]
fn serial_number() -> Result<String> {
    let dmidecode = Command::new("dmidecode")
        .args(["-s", "system-serial-number"])
        .output()?;
    Ok(String::from_utf8(dmidecode.stdout)?)
}

/// exec("system_profiler SPHardwareDataType | awk '/Serial/ {print $4}'")
#[cfg(target_os = "macos")]
pub fn serial_number() -> Result<String> {
    let system_profiler = Command::new("system_profiler")
        .arg("SPHardwareDataType")
        .stdout(Stdio::piped())
        .spawn()?;
    let awk = Command::new("awk")
        .arg("/Serial/ {print $4}")
        .stdin(Stdio::from(system_profiler.stdout.unwrap()))
        .output()?;
    Ok(String::from_utf8(awk.stdout)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_number() {
        let serial = serial_number().unwrap();
        assert!(!serial.is_empty());
    }
}
