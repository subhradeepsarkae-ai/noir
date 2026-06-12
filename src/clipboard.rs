use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn get_paths() -> Result<Vec<String>> {
    #[cfg(windows)]
    {
        get_paths_windows()
    }
    #[cfg(target_os = "linux")]
    {
        get_paths_linux()
    }
    #[cfg(target_os = "macos")]
    {
        get_paths_macos()
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        Ok(vec![])
    }
}

#[cfg(windows)]
fn get_paths_windows() -> Result<Vec<String>> {
    // Method 1: FileDropList — what Explorer uses when you Ctrl+C a file
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "$f=Get-Clipboard -Format FileDropList -ErrorAction SilentlyContinue; if($f -and $f.Count -gt 0){$f | ForEach { $_.FullName }}",
        ])
        .output()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        let paths: Vec<String> = text
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && Path::new(l).exists())
            .collect();
        if !paths.is_empty() {
            return Ok(paths);
        }
    }

    // Method 2: Plain text clipboard (e.g., "path" | Set-Clipboard)
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Get-Clipboard"])
        .output()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() {
            let cleaned = text.trim_matches('"');
            if Path::new(cleaned).exists() {
                return Ok(vec![cleaned.to_string()]);
            }
            if let Some(decoded) = cleaned.strip_prefix("file://") {
                if Path::new(decoded).exists() {
                    return Ok(vec![decoded.to_string()]);
                }
            }
        }
    }

    Ok(vec![])
}

#[cfg(target_os = "linux")]
fn get_paths_linux() -> Result<Vec<String>> {
    for (cmd, args) in &[
        ("xclip", &["-o", "-sel", "clipboard"] as &[&str]),
        ("wl-paste", &[] as &[&str]),
    ] {
        if let Ok(output) = Command::new(cmd).args(args).output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    let cleaned = text.trim_matches('"');
                    if Path::new(cleaned).exists() {
                        return Ok(vec![cleaned.to_string()]);
                    }
                }
            }
        }
    }
    Ok(vec![])
}

#[cfg(target_os = "macos")]
fn get_paths_macos() -> Result<Vec<String>> {
    let output = Command::new("pbpaste").output()?;
    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        if !text.is_empty() {
            let cleaned = text.trim_matches('"');
            if Path::new(cleaned).exists() {
                return Ok(vec![cleaned.to_string()]);
            }
        }
    }
    Ok(vec![])
}
