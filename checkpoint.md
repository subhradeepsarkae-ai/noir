# noir — checkpoint

**Binary:** `nr` (493 KB, single-file, no deps beyond clap+anyhow)

## Status (June 2026)

All core features are implemented and tested. The binary auto-downloads ffmpeg on first run, detects file types by magic bytes, and converts between any compatible formats.

## Project Structure

```
D:\Products\noir\
├── Cargo.toml          # Config: LTO, opt-level=z, panic=abort
├── checkpoint.md       # This file
├── .gitignore
├── src/
│   ├── main.rs         # Entry, arg resolution, orchestration
│   ├── cli.rs          # Clap parser (arg1, arg2, --list, --convert, --force)
│   ├── detect.rs       # Magic-byte + extension media type detection
│   ├── registry.rs     # 96+ format targets with ffmpeg args
│   ├── clipboard.rs    # Multi-file clipboard (FileDropList + text fallback)
│   ├── ffmpeg.rs       # Auto-setup (PATH/cache/download) + conversion
│   └── interactive.rs  # Terminal menu: pick format by # or name
└── target/release/nr.exe
```

## Features

### CLI Usage

```
nr                      clipboard file -> interactive format picker
nr mp4                  clipboard file -> convert to mp4
nr video.mov            detect file -> interactive format picker
nr video.mov mkv        convert video.mov to mkv
nr --list               show all supported formats
nr --convert            clipboard file -> interactive menu (same as no-arg)
nr --force mp4          batch convert ALL clipboard files to mp4
```

### Arg Auto-Detection (single arg)
1. File exists on disk? → treat as file
2. Known format name? → treat as format (read clipboard for file)
3. Contains a dot? → assume file, error if not found
4. Else → assume format, read clipboard

### Format Registry — 109 total targets

| Category | Count | Coverage |
|----------|-------|----------|
| Video → video containers | 24 | mp4/h265/mkv/webm/vp9/avi/mov/wmv/flv/m4v/ogv/mpg/ts/3gp/3g2/gif/dv/m2ts/vob/av1/mxf/dnxhd/mjpeg/nut/psp |
| Video → audio extraction | 23 | mp3/wav/flac/ogg/aac/adts/opus/wma/m4a/ac3/eac3/truehd/aiff/au/caf/voc/w64/tta/amr/gsm/ircam/sox |
| Audio → audio | 24 | mp3/wav/flac/ogg/aac/adts/opus/spx/wma/m4a/ac3/eac3/truehd/aiff/au/caf/voc/w64/tta/amr/gsm/ircam/sox/oga |
| Image → image | 25 | jpg/png/apng/webp/avif/bmp/tiff/gif/ico/ppm/pgm/pbm/pam/pcx/tga/xbm/xwd/dpx/sgi/jp2/hdr/sun/wbmp/pfm/fits |
| Image → video slideshow | 7 | mp4/mkv/webm/vp9/avi/mov/gif (5s slides, 2s for gif) |
| Audio → video container | 6 | mp4/mkv/webm/avi/mov/vp9 (blank black frame + audio) |

Variant formats shown in --list: `h265 → .mp4`, `vp9 → .webm`, `av1 → .mkv`, `psp → .mp4`

**Not available** (not in ffmpeg build): wavpack encoder, dts/truehd standalone, alac standalone container.

### File Detection (detect.rs)
- Magic bytes for 30+ formats (ftyp/mp4, matroska/mkv, riff/avi, riff/wav, riff/webp, OggS, FLV, ID3, mp3 sync word, fLaC, JPEG, PNG, GIF, BMP, TIFF, HEIC/HEIF/AVIF)
- Extension fallback for common types
- Returns Video/Audio/Image category

### Clipboard (clipboard.rs)
- **Windows:** FileDropList first (Explorer Ctrl+C → all files), then plain text
- **Linux:** xclip, then wl-paste
- **macOS:** pbpaste
- Returns `Vec<String>` (supports multiple files from Explorer multi-select)

### ffmpeg Auto-Setup (ffmpeg.rs)
1. Check PATH
2. Check `%LOCALAPPDATA%/noir/ffmpeg/ffmpeg.exe` (Win) or `~/.cache/noir/ffmpeg/ffmpeg`
3. Download + extract if missing
   - Win: gyan.dev essentials.zip via PowerShell
   - Linux: johnvansickle.com static build or apt/pacman/dnf
   - macOS: evermeet.cx or homebrew
4. Piped stderr (shown only on failure)

### Interactive Menu (interactive.rs)
- Clears screen, shows filename + size + detected type
- Numbered format list
- Accepts # or format name
- q/quit/exit to quit

### Binary Size Optimization
- `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`, `strip = true`, `panic = "abort"`
- Zero TUI framework (pure println! + stdin)
- Platform-native clipboard (no arboard crate, saves ~1MB)
- Custom magic-byte detection (no infer crate)

## Recent Changes

| Date | Change |
|------|--------|
| June 12 | Added `--force <format>` for batch clipboard conversion |
| June 12 | Clipboard returns `Vec<String>` (multi-file support) |
| June 12 | Cross-type conversions: Image→video slideshow, Audio→video blank frame |
| June 12 | Added `pre_args` to FormatTarget for pre-input ffmpeg args |
| June 12 | Created checkpoint.md |

## Pending / Future Work

- [ ] `--version` flag (already handled by clap derive)
- [ ] Progress bar during ffmpeg conversion (stderr parsing for duration)
- [ ] Make slideshow duration configurable (not hardcoded 5s)
- [ ] Support multiple input files with wildcards (`nr *.jpg mp4`)
- [ ] Add `nr <url>` support (download + convert)
- [ ] Windows Explorer context menu integration
- [ ] Drag-and-drop onto binary
- [ ] Config file for custom ffmpeg args presets
- [ ] Batch/parallel conversion with optional concurrency flag
