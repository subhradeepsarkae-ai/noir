use crate::registry::FormatTarget;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn find_or_setup_ffmpeg() -> Result<PathBuf> {
    if is_on_path() {
        return Ok(PathBuf::from("ffmpeg"));
    }

    let cache = cache_path();
    let cached = cache.join(binary_name());
    if cached.exists() {
        return Ok(cached);
    }

    download_ffmpeg(&cache)
}

fn is_on_path() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn cache_path() -> PathBuf {
    #[cfg(windows)]
    {
        let local = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| {
            let user = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into());
            format!("{}\\AppData\\Local", user)
        });
        PathBuf::from(local).join("noir").join("ffmpeg")
    }
    #[cfg(unix)]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        if cfg!(target_os = "macos") {
            PathBuf::from(&home)
                .join("Library")
                .join("Caches")
                .join("noir")
                .join("ffmpeg")
        } else {
            PathBuf::from(&home).join(".cache").join("noir").join("ffmpeg")
        }
    }
}

fn binary_name() -> &'static str {
    if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    }
}

fn download_ffmpeg(cache: &Path) -> Result<PathBuf> {
    #[cfg(windows)]
    {
        download_windows(cache)
    }
    #[cfg(target_os = "linux")]
    {
        download_linux(cache)
    }
    #[cfg(target_os = "macos")]
    {
        download_macos(cache)
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        anyhow::bail!("Unsupported platform. Install ffmpeg manually: https://ffmpeg.org/download.html");
    }
}

#[cfg(windows)]
fn download_windows(cache: &Path) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join("noir_ffmpeg_dl");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir)?;

    let zip_path = temp_dir.join("ffmpeg.zip");
    let extract_dir = temp_dir.join("extracted");

    println!("ffmpeg not found. Downloading...");

    let script = format!(
        "$ProgressPreference='SilentlyContinue'; \
         Invoke-WebRequest -Uri 'https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip' \
         -OutFile '{}'", 
        zip_path.display()
    );

    let status = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .status()
        .context("Failed to download ffmpeg")?;

    if !status.success() {
        let _ = std::fs::remove_dir_all(&temp_dir);
        anyhow::bail!("Download failed. Install ffmpeg manually from https://ffmpeg.org/download.html");
    }

    println!("Extracting...");
    std::fs::create_dir_all(&extract_dir)?;
    let extract_script = format!(
        "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
        zip_path.display(),
        extract_dir.display()
    );
    let status = Command::new("powershell")
        .args(["-NoProfile", "-Command", &extract_script])
        .status()
        .context("Failed to extract ffmpeg")?;

    if !status.success() {
        let _ = std::fs::remove_dir_all(&temp_dir);
        anyhow::bail!("Extraction failed. Install ffmpeg manually.");
    }

    let found = find_file_recursive(&extract_dir, "ffmpeg.exe")
        .context("ffmpeg.exe not found after extraction")?;

    std::fs::create_dir_all(cache)?;
    let dest = cache.join("ffmpeg.exe");
    std::fs::copy(&found, &dest)
        .context("Failed to copy ffmpeg to cache")?;

    let _ = std::fs::remove_dir_all(&temp_dir);
    println!("✓ ffmpeg installed to {}", dest.display());

    Ok(dest)
}

#[cfg(target_os = "linux")]
fn download_linux(cache: &Path) -> Result<PathBuf> {
    if Command::new("apt").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        println!("Installing ffmpeg via apt...");
        let status = Command::new("sudo")
            .args(["apt", "install", "-y", "ffmpeg"])
            .status()
            .context("Failed to install ffmpeg via apt")?;
        if status.success() {
            return Ok(PathBuf::from("ffmpeg"));
        }
    }
    if Command::new("pacman").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        println!("Installing ffmpeg via pacman...");
        let status = Command::new("sudo")
            .args(["pacman", "-S", "--noconfirm", "ffmpeg"])
            .status()
            .context("Failed to install ffmpeg via pacman")?;
        if status.success() {
            return Ok(PathBuf::from("ffmpeg"));
        }
    }
    if Command::new("dnf").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
        println!("Installing ffmpeg via dnf...");
        let status = Command::new("sudo")
            .args(["dnf", "install", "-y", "ffmpeg"])
            .status()
            .context("Failed to install ffmpeg via dnf")?;
        if status.success() {
            return Ok(PathBuf::from("ffmpeg"));
        }
    }

    // Fallback: download static build
    let temp = std::env::temp_dir().join("noir_ffmpeg_dl");
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp)?;

    println!("Downloading ffmpeg static build...");
    let status = Command::new("curl")
        .args([
            "-L",
            "-o",
            &temp.join("ffmpeg.tar.xz").to_string_lossy(),
            "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz",
        ])
        .status()
        .context("Failed to download ffmpeg")?;

    if !status.success() {
        let _ = std::fs::remove_dir_all(&temp);
        anyhow::bail!("Download failed. Install ffmpeg via your package manager.");
    }

    println!("Extracting...");
    let status = Command::new("tar")
        .args([
            "-xf",
            &temp.join("ffmpeg.tar.xz").to_string_lossy(),
            "-C",
            &temp.to_string_lossy(),
        ])
        .status()
        .context("Failed to extract ffmpeg")?;

    if !status.success() {
        let _ = std::fs::remove_dir_all(&temp);
        anyhow::bail!("Extraction failed.");
    }

    let found =
        find_file_recursive(&temp, "ffmpeg").context("ffmpeg binary not found after extraction")?;

    std::fs::create_dir_all(cache)?;
    let dest = cache.join("ffmpeg");
    std::fs::copy(&found, &dest)?;
    std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;

    let _ = std::fs::remove_dir_all(&temp);
    println!("✓ ffmpeg installed to {}", dest.display());

    Ok(dest)
}

#[cfg(target_os = "macos")]
fn download_macos(cache: &Path) -> Result<PathBuf> {
    if Command::new("brew")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        println!("Installing ffmpeg via Homebrew...");
        let status = Command::new("brew")
            .args(["install", "ffmpeg"])
            .status()
            .context("Failed to install ffmpeg via brew")?;
        if status.success() {
            return Ok(PathBuf::from("ffmpeg"));
        }
    }

    // Fallback: download from evermeet.cx
    let temp = std::env::temp_dir().join("noir_ffmpeg_dl");
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp)?;

    println!("Downloading ffmpeg...");
    let status = Command::new("curl")
        .args([
            "-L",
            "-o",
            &temp.join("ffmpeg.zip").to_string_lossy(),
            "https://evermeet.cx/ffmpeg/ffmpeg-7.1.zip",
        ])
        .status()
        .context("Failed to download ffmpeg")?;

    if !status.success() {
        let _ = std::fs::remove_dir_all(&temp);
        anyhow::bail!("Download failed. Install ffmpeg via 'brew install ffmpeg'.");
    }

    println!("Extracting...");
    let status = Command::new("unzip")
        .args([
            "-o",
            &temp.join("ffmpeg.zip").to_string_lossy(),
            "-d",
            &temp.to_string_lossy(),
        ])
        .status()
        .context("Failed to extract ffmpeg")?;

    if !status.success() {
        let _ = std::fs::remove_dir_all(&temp);
        anyhow::bail!("Extraction failed.");
    }

    let found =
        find_file_recursive(&temp, "ffmpeg").context("ffmpeg binary not found after extraction")?;

    std::fs::create_dir_all(cache)?;
    let dest = cache.join("ffmpeg");
    std::fs::copy(&found, &dest)?;
    std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;

    let _ = std::fs::remove_dir_all(&temp);
    println!("✓ ffmpeg installed to {}", dest.display());

    Ok(dest)
}

pub fn convert(ffmpeg: &Path, input: &Path, output: &Path, target: &FormatTarget) -> Result<()> {
    let res = Command::new(ffmpeg)
        .arg("-y")
        .args(target.pre_args)
        .arg("-i")
        .arg(input)
        .args(target.post_args)
        .arg(output)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output()
        .context("Failed to execute ffmpeg")?;

    if !res.status.success() {
        let stderr = String::from_utf8_lossy(&res.stderr);
        anyhow::bail!("ffmpeg failed:\n{}", stderr);
    }

    Ok(())
}

fn find_file_recursive(dir: &Path, name: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }
    let mut entries = std::fs::read_dir(dir).ok()?;
    while let Some(entry) = entries.next() {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.file_name()?.to_str()? == name {
            return Some(path);
        }
        if path.is_dir() {
            if let Some(found) = find_file_recursive(&path, name) {
                return Some(found);
            }
        }
    }
    None
}
