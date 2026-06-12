use crate::detect::MediaType;
use crate::registry::FormatTarget;
use anyhow::Result;
use std::io::{self, Write};
use std::path::Path;

pub fn choose_target(file: &Path, media_type: &MediaType, targets: &[FormatTarget]) -> Result<String> {
    loop {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().ok();

        let file_name = file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?");
        let file_size = format_size(file).unwrap_or_default();

        println!("── nr ── Universal Converter ──────────────────────────");
        println!("  {}  ({})  —  {}", file_name, file_size, media_type.label());
        println!();

        if targets.is_empty() {
            println!("  No conversion targets available for this file type.");
            return Err(anyhow::anyhow!("Unsupported file type"));
        }

        println!("  Available formats:");
        for (i, target) in targets.iter().enumerate() {
            println!("  {:<2}) .{:<4}  {}", i + 1, target.ext, target.label);
        }
        println!();
        print!("  Enter # or format (q to quit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input == "q" || input == "quit" || input == "exit" {
            std::process::exit(0);
        }

        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= targets.len() {
                return Ok(targets[num - 1].ext.to_string());
            }
        }

        if targets.iter().any(|t| t.ext == input) {
            return Ok(input);
        }

        print!("  Invalid. Press Enter to try again...");
        io::stdout().flush()?;
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp)?;
    }
}

fn format_size(path: &Path) -> Option<String> {
    let bytes = std::fs::metadata(path).ok()?.len();
    if bytes == 0 {
        return None;
    }
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size > 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    Some(if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    })
}
