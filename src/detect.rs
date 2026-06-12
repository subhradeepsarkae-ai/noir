use anyhow::{Context, Result};
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaType {
    Video,
    Audio,
    Image,
}

impl MediaType {
    pub fn label(&self) -> &'static str {
        match self {
            MediaType::Video => "Video",
            MediaType::Audio => "Audio",
            MediaType::Image => "Image",
        }
    }
}

pub fn detect_media_type(path: &Path) -> Result<MediaType> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if let Some(mt) = check_extension(&ext) {
        return Ok(mt);
    }

    let mut buf = [0u8; 16];
    let mut file = std::fs::File::open(path)
        .with_context(|| format!("Cannot open {}", path.display()))?;
    let n = file
        .read(&mut buf)
        .with_context(|| format!("Cannot read {}", path.display()))?;
    drop(file);

    if n < 4 {
        return Err(anyhow::anyhow!("File too small to detect type"));
    }

    if let Some(mt) = check_magic(&buf, n) {
        return Ok(mt);
    }

    if let Some(mt) = check_extension(&ext) {
        return Ok(mt);
    }

    Err(anyhow::anyhow!("Unrecognized file type"))
}

fn check_extension(ext: &str) -> Option<MediaType> {
    match ext {
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg"
        | "ts" | "ogv" | "3gp" | "3g2" | "vob" => Some(MediaType::Video),
        "mp3" | "wav" | "flac" | "ogg" | "aac" | "wma" | "m4a" | "opus" | "alac" => {
            Some(MediaType::Audio)
        }
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "ico" | "heic"
        | "heif" | "avif" => Some(MediaType::Image),
        _ => None,
    }
}

fn check_magic(buf: &[u8; 16], len: usize) -> Option<MediaType> {
    if len >= 12 && &buf[4..8] == b"ftyp" {
        let brand = &buf[8..12];
        if brand == b"heic"
            || brand == b"heif"
            || brand == b"mif1"
            || brand == b"msf1"
            || brand == b"avif"
        {
            return Some(MediaType::Image);
        }
        return Some(MediaType::Video);
    }

    if len >= 4 && buf[0..4] == [0x1A, 0x45, 0xDF, 0xA3] {
        return Some(MediaType::Video);
    }
    if len >= 12 && &buf[0..4] == b"RIFF" && &buf[8..12] == b"AVI " {
        return Some(MediaType::Video);
    }
    if len >= 4 && buf[0..4] == [0x30, 0x26, 0xB2, 0x75] {
        return Some(MediaType::Video);
    }
    if len >= 3 && &buf[0..3] == b"FLV" {
        return Some(MediaType::Video);
    }
    if len >= 4 && &buf[0..4] == b"OggS" {
        return Some(MediaType::Video);
    }
    if len >= 3 && &buf[0..3] == b"ID3" {
        return Some(MediaType::Audio);
    }
    if len >= 2 && buf[0] == 0xFF && (buf[1] & 0xE0) == 0xE0 {
        return Some(MediaType::Audio);
    }
    if len >= 12 && &buf[0..4] == b"RIFF" && &buf[8..12] == b"WAVE" {
        return Some(MediaType::Audio);
    }
    if len >= 4 && &buf[0..4] == b"fLaC" {
        return Some(MediaType::Audio);
    }
    if len >= 3 && buf[0..3] == [0xFF, 0xD8, 0xFF] {
        return Some(MediaType::Image);
    }
    if len >= 8 && &buf[0..8] == b"\x89PNG\r\n\x1a\n" {
        return Some(MediaType::Image);
    }
    if len >= 4 && &buf[0..3] == b"GIF" && (buf[3] == b'7' || buf[3] == b'9') {
        return Some(MediaType::Image);
    }
    if len >= 2 && &buf[0..2] == b"BM" {
        return Some(MediaType::Image);
    }
    if len >= 12 && &buf[0..4] == b"RIFF" && &buf[8..12] == b"WEBP" {
        return Some(MediaType::Image);
    }
    if len >= 4 {
        if (&buf[0..2] == b"II" && buf[2] == 0x2A && buf[3] == 0x00)
            || (&buf[0..2] == b"MM" && buf[2] == 0x00 && buf[3] == 0x2A)
        {
            return Some(MediaType::Image);
        }
    }

    None
}
