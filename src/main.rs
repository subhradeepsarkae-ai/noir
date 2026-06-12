mod cli;
mod clipboard;
mod detect;
mod ffmpeg;
mod interactive;
mod registry;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::Path;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        for cause in e.chain().skip(1) {
            eprintln!("  -> {}", cause);
        }
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();

    if cli.list {
        registry::print_all();
        return Ok(());
    }

    // --force <format> — batch convert all clipboard files
    if let Some(ref force_ext) = cli.force {
        let files = clipboard_files()?;
        if files.is_empty() {
            anyhow::bail!("No files in clipboard to convert.");
        }
        let force_ext = force_ext.to_lowercase();
        for file_path in &files {
            let path = Path::new(file_path);
            let media_type = detect::detect_media_type(path)
                .with_context(|| format!("Cannot detect type of {}", path.display()))?;
            let target = registry::resolve_target(&media_type, &force_ext)?;
            convert_file(path, target)?;
        }
        return Ok(());
    }

    // --convert forces interactive menu from clipboard (single file)
    if cli.convert {
        let files = clipboard_files()?;
        let file_path = first_clipboard_file(files)?;
        let path = std::path::Path::new(&file_path);
        let media_type = detect::detect_media_type(path)
            .with_context(|| format!("Cannot detect type of {}", path.display()))?;
        let targets = registry::get_targets(&media_type);
        if targets.is_empty() {
            anyhow::bail!("No conversions available for {} files", media_type.label());
        }
        let target_ext = interactive::choose_target(path, &media_type, targets)?;
        let target = targets.iter().find(|t| t.ext == target_ext).unwrap();
        return convert_file(path, target);
    }

    // Resolve file path and optional target format from args
    let (file_path, target_ext) = resolve_args(&cli)?;
    let path = Path::new(&file_path);

    if !path.exists() {
        anyhow::bail!("File not found: {}", path.display());
    }

    let media_type = detect::detect_media_type(path)
        .with_context(|| format!("Cannot detect type of {}", path.display()))?;

    let targets = registry::get_targets(&media_type);

    if targets.is_empty() {
        anyhow::bail!("No conversions available for {} files", media_type.label());
    }

    let target_ext = if let Some(ref ext) = target_ext {
        let ext = ext.to_lowercase();
        if !targets.iter().any(|t| t.ext == ext) {
            anyhow::bail!(
                "'{}' is not supported for {} files. Use --list to see options.",
                ext,
                media_type.label()
            );
        }
        ext
    } else {
        interactive::choose_target(path, &media_type, targets)?
    };

    let target = targets.iter().find(|t| t.ext == target_ext).unwrap();
    convert_file(path, target)
}

fn resolve_args(cli: &cli::Cli) -> Result<(String, Option<String>)> {
    match (&cli.arg1, &cli.arg2) {
        // nr file.mov mp4
        (Some(a), Some(b)) => Ok((a.clone(), Some(b.clone()))),

        // nr (no args) — try clipboard
        (None, None) => {
            let files = clipboard_files()?;
            let file = first_clipboard_file(files)?;
            Ok((file, None))
        }

        // arg2 without arg1 — shouldn't happen with clap, but handle it
        (None, Some(b)) => {
            let files = clipboard_files()?;
            let file = first_clipboard_file(files)?;
            Ok((file, Some(b.clone())))
        }

        // nr <single arg>
        (Some(arg), None) => {
            let path = Path::new(arg);

            // If it exists on disk, treat as file
            if path.exists() {
                return Ok((arg.clone(), None));
            }

            // If it looks like a known format, treat as format (use clipboard)
            if cli::is_format_name(&arg.to_lowercase()) {
                let files = clipboard_files()?;
                let file = first_clipboard_file(files)?;
                return Ok((file, Some(arg.clone())));
            }

            // If it has a dot (file extension), try as file
            if arg.contains('.') {
                anyhow::bail!("File not found: {}", arg);
            }

            // Unknown string — assume it's a format, get file from clipboard
            let files = clipboard_files()?;
            let file = first_clipboard_file(files)?;
            Ok((file, Some(arg.clone())))
        }
    }
}

fn clipboard_files() -> Result<Vec<String>> {
    let paths = clipboard::get_paths()?;
    if paths.is_empty() {
        anyhow::bail!(
            "No files in clipboard.\n\
             Copy a file path to clipboard, or:\n  \
             nr <file>              — detect file\n  \
             nr <file> <format>     — convert specific file\n  \
             nr --list              — show all formats"
        );
    }
    Ok(paths)
}

fn first_clipboard_file(files: Vec<String>) -> Result<String> {
    Ok(files[0].clone())
}

fn convert_file(input: &Path, target: &registry::FormatTarget) -> Result<()> {
    let ffmpeg = ffmpeg::find_or_setup_ffmpeg()?;

    let mut output = input.to_path_buf();
    output.set_extension(target.output_ext);

    if output == input {
        let ext = target.output_ext;
        println!("File is already .{} — nothing to do.", ext);
        return Ok(());
    }

    println!(
        "Converting: {}  →  {}",
        input.display(),
        output.display()
    );
    let start = std::time::Instant::now();

    ffmpeg::convert(&ffmpeg, input, &output, target)?;

    let elapsed = start.elapsed();
    let size = std::fs::metadata(&output)
        .map(|m| format_size(m.len()))
        .unwrap_or_default();

    println!();
    println!("  ✓ Done!  ({:.1}s)", elapsed.as_secs_f64());
    println!("    Location: {}", output.display());
    if !size.is_empty() {
        println!("    Size:     {}", size);
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size > 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}
