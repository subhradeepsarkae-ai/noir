# nr — Universal Media Converter

**One binary. Zero dependencies. Any format. 493 KB.**

```powershell
# Windows (PowerShell)
iex "& { $(irm https://raw.githubusercontent.com/subhradeepsarkae-ai/noir/main/scripts/install.ps1) }"
```

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/subhradeepsarkae-ai/noir/main/scripts/install.sh | bash
```

`nr` is a CLI tool that converts any media file (video, audio, image) to any compatible format using ffmpeg — but you never install ffmpeg yourself. It auto-downloads and caches it on first run.

---

## Quick Start

```bash
# Convert clipboard file (Ctrl+C in Explorer) — pick format interactively
nr

# Convert clipboard file directly to mp4
nr mp4

# Convert a specific file — pick format
nr video.mov

# Convert a specific file to a specific format
nr video.mov mkv

# Batch convert all clipboard files to one format
nr --force mp4

# Show all 109 supported conversion targets
nr --list
```

## Features

### 📦 Zero-Install ffmpeg
The first time you run `nr`, it downloads ffmpeg automatically:
- **Windows:** gyan.dev essentials build (includes libx264, libx265, libvpx-vp9, libaom-av1, libmp3lame, libopus, libvorbis, libtheora, and more)
- **Linux:** johnvansickle.com static build (or apt/pacman/dnf)
- **macOS:** evermeet.cx build (or homebrew)

Cached at `%LOCALAPPDATA%\noir\ffmpeg\` (Win) or `~/.cache/noir/ffmpeg/` (Linux/macOS).

### 🧠 Smart File Detection
Detects media type by **magic bytes** (not just extension): MP4, MKV, AVI, MOV, WMV, FLV, WebM, Ogg, MP3, WAV, FLAC, JPEG, PNG, GIF, BMP, TIFF, HEIC/HEIF/AVIF, and more.

### 📋 Multi-File Clipboard
Explorer Ctrl+C captures all selected files. `nr --force mp4` converts every one.

### 🎯 109 Conversion Targets

| Category | Targets |
|----------|---------|
| Video → video | mp4, h265, mkv, webm, vp9, avi, mov, wmv, flv, m4v, ogv, mpg, ts, 3gp, 3g2, gif, dv, m2ts, vob, av1, mxf, dnxhd, mjpeg, nut, psp |
| Video → audio | mp3, wav, flac, ogg, aac, opus, wma, m4a, ac3, eac3, truehd, aiff, and more |
| Audio → audio | mp3, wav, flac, ogg, aac, opus, spx, wma, m4a, ac3, eac3, truehd, aiff, and more |
| Image → image | jpg, png, webp, avif, bmp, tiff, gif, ico, jp2, ppm, pcx, tga, and more |
| Image → video | mp4, mkv, webm, vp9, avi, mov, gif (5s slideshow) |
| Audio → video | mp4, mkv, webm, avi, mov, vp9 (blank frame + audio) |

## Examples

```bash
# Extract audio from video
nr video.mp4 mp3

# Create a WebM with the best compression
nr video.mov vp9

# Convert an image to multiple formats for the web
nr --force webp       # converts all clipboard images to WebP

# Make a video from a photo
nr photo.jpg mp4      # produces a 5-second slideshow

# Wrap audio in a video container (e.g., for upload sites that require video)
nr song.mp3 mov

# Lossless remux to MKV
nr video.mov mkv
```

## Install

### Windows (PowerShell)
```powershell
iex "& { $(irm https://raw.githubusercontent.com/subhradeepsarkae-ai/noir/main/scripts/install.ps1) }"
```

### Linux / macOS
```bash
curl -sSL https://raw.githubusercontent.com/subhradeepsarkae-ai/noir/main/scripts/install.sh | bash
```

### Build from source
```bash
cargo install --git https://github.com/subhradeepsarkae-ai/noir nr
```
Requires [Rust](https://rustup.rs).

### Manual
1. Download `nr.exe` (Windows) or `nr` (Linux/macOS) from [Releases](https://github.com/subhradeepsarkae-ai/noir/releases)
2. Place it in a directory on your `PATH`

## Build

```bash
git clone https://github.com/subhradeepsarkae-ai/noir.git
cd noir
cargo build --release
# Binary: target/release/nr.exe (or target/release/nr on Unix)
```

Strip: `~493 KB` with LTO, `opt-level=z`, `panic=abort`.

## How It Works

```
┌──────────┐    ┌──────────┐    ┌───────────┐    ┌──────────┐
│ Clipboard│───>│ Detect   │───>│ Pick      │───>│ ffmpeg   │
│ or file  │    │ magic    │    │ format    │    │ convert  │
│ arg      │    │ bytes    │    │ interact. │    │          │
└──────────┘    └──────────┘    └───────────┘    └──────────┘
```

1. You provide a file (via clipboard or argument)
2. `nr` reads magic bytes to determine Video/Audio/Image
3. You pick a target format (interactive menu or `--force`)
4. ffmpeg (auto-downloaded if missing) handles the conversion

## Why nr?

- **No runtime dependencies** — not even ffmpeg (it's auto-bundled)
- **Tiny binary** — 493 KB, written in Rust
- **Smart defaults** — sensible codec choices for each format
- **Clipboard-first** — Ctrl+C in Explorer, then `nr mp4`
- **Batch friendly** — `--force` converts entire clipboard at once

## License

MIT
