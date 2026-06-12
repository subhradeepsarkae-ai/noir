use crate::detect::MediaType;
use anyhow::Result;

pub struct FormatTarget {
    pub ext: &'static str,
    pub label: &'static str,
    pub output_ext: &'static str,
    pub pre_args: &'static [&'static str],
    pub post_args: &'static [&'static str],
}

macro_rules! ft {
    ($ext:expr, $label:expr, $args:expr) => {
        FormatTarget { ext: $ext, label: $label, output_ext: $ext, pre_args: &[], post_args: $args }
    };
    ($ext:expr, $label:expr, $output:expr, $args:expr) => {
        FormatTarget { ext: $ext, label: $label, output_ext: $output, pre_args: &[], post_args: $args }
    };
}

macro_rules! ft_pre {
    ($ext:expr, $label:expr, $pre:expr, $args:expr) => {
        FormatTarget { ext: $ext, label: $label, output_ext: $ext, pre_args: $pre, post_args: $args }
    };
    ($ext:expr, $label:expr, $output:expr, $pre:expr, $args:expr) => {
        FormatTarget { ext: $ext, label: $label, output_ext: $output, pre_args: $pre, post_args: $args }
    };
}

// ── Video input → all video + audio formats ──
static VIDEO_TARGETS: &[FormatTarget] = &[
    ft!("mp4",      "MP4 H.264/AAC",                 &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac", "-movflags", "+faststart"]),
    ft!("h265",     "MP4 H.265/HEVC/AAC",  "mp4",    &["-c:v", "libx265", "-preset", "fast", "-c:a", "aac", "-movflags", "+faststart"]),
    ft!("mkv",      "MKV (stream copy, lossless)",   &["-c", "copy"]),
    ft!("webm",     "WebM VP8/Vorbis",               &["-c:v", "libvpx", "-c:a", "libvorbis"]),
    ft!("vp9",      "WebM VP9/Opus",       "webm",   &["-c:v", "libvpx-vp9", "-c:a", "libopus", "-crf", "30", "-b:v", "0"]),
    ft!("avi",      "AVI MPEG-4/MP3",                &["-c:v", "mpeg4", "-c:a", "mp3"]),
    ft!("mov",      "MOV H.264/AAC",                 &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac"]),
    ft!("wmv",      "WMV WMV2/WMAv2",                &["-c:v", "wmv2", "-c:a", "wmav2"]),
    ft!("flv",      "FLV H.264/MP3",                 &["-c:v", "libx264", "-c:a", "mp3", "-flvflags", "add_keyframe_index"]),
    ft!("m4v",      "M4V H.264/AAC",                 &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac", "-movflags", "+faststart"]),
    ft!("ogv",      "OGV Theora/Vorbis",             &["-c:v", "libtheora", "-c:a", "libvorbis"]),
    ft!("mpg",      "MPG MPEG-2/MP2",                &["-c:v", "mpeg2video", "-c:a", "mp2"]),
    ft!("ts",       "MPEG-TS H.264/AAC",             &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac"]),
    ft!("3gp",      "3GP H.263/AAC",                 &["-c:v", "h263", "-c:a", "aac"]),
    ft!("3g2",      "3G2 H.263/AAC",                 &["-c:v", "h263", "-c:a", "aac"]),
    ft!("gif",      "GIF animated (10fps, 480w)",    &["-vf", "fps=10,scale=480:-1"]),
    ft!("dv",       "DV DV/PCM",                     &["-c:v", "dvvideo", "-c:a", "pcm_s16le"]),
    ft!("m2ts",     "M2TS H.264/AAC",                &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac"]),
    ft!("vob",      "VOB MPEG-2/AC3",                &["-c:v", "mpeg2video", "-c:a", "ac3"]),
    ft!("av1",      "AV1 AV1/Opus (slow)", "mkv",    &["-c:v", "libaom-av1", "-crf", "30", "-c:a", "libopus", "-cpu-used", "4"]),
    ft!("mxf",      "MXF MPEG-2/PCM",                &["-c:v", "mpeg2video", "-c:a", "pcm_s16le"]),
    ft!("dnxhd",    "DNxHD (professional)",          &["-c:v", "dnxhd", "-b:v", "36M", "-pix_fmt", "yuv422p"]),
    ft!("mjpeg",    "Motion JPEG AVI",               &["-c:v", "mjpeg", "-q:v", "3"]),
    ft!("nut",      "NUT H.264/AAC",                 &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac"]),
    ft!("psp",      "PSP MP4 H.264/AAC", "mp4",      &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac", "-f", "psp"]),
    // === Audio-only extraction (raw codec streams) ===
    ft!("mp3",      "MP3 audio extract",              &["-vn", "-c:a", "libmp3lame", "-q:a", "2"]),
    ft!("wav",      "WAV PCM audio extract",          &["-vn", "-c:a", "pcm_s16le"]),
    ft!("flac",     "FLAC lossless audio extract",    &["-vn", "-c:a", "flac"]),
    ft!("ogg",      "OGG Vorbis audio extract",       &["-vn", "-c:a", "libvorbis"]),
    ft!("aac",      "AAC audio extract (192k)",       &["-vn", "-c:a", "aac", "-b:a", "192k"]),
    ft!("adts",     "ADTS AAC audio extract",         &["-vn", "-c:a", "aac"]),
    ft!("opus",     "Opus audio extract",             &["-vn", "-c:a", "libopus"]),
    ft!("wma",      "WMA audio extract",              &["-vn", "-c:a", "wmav2"]),
    ft!("m4a",      "M4A AAC audio extract",          &["-vn", "-c:a", "aac", "-b:a", "192k", "-movflags", "+faststart"]),
    ft!("ac3",      "AC3 Dolby audio extract",        &["-vn", "-c:a", "ac3"]),
    ft!("eac3",     "E-AC-3 Dolby Digital+ extract",  &["-vn", "-c:a", "eac3"]),
    ft!("truehd",   "TrueHD audio extract",           &["-vn", "-c:a", "truehd"]),
    ft!("aiff",     "AIFF PCM audio extract",         &["-vn", "-c:a", "pcm_s16be"]),
    ft!("au",       "AU Sun audio extract",           &["-vn", "-c:a", "pcm_s16be"]),
    ft!("caf",      "CAF Core Audio extract",         &["-vn", "-c:a", "pcm_s16le"]),
    ft!("voc",      "VOC Creative Voice extract",     &["-vn", "-c:a", "pcm_s16le"]),
    ft!("w64",      "Wave64 audio extract",           &["-vn", "-c:a", "pcm_s16le"]),
    ft!("tta",      "TTA True Audio extract",         &["-vn", "-c:a", "tta"]),
    ft!("amr",      "AMR narrowband extract",         &["-vn", "-c:a", "libopencore_amrnb", "-ar", "8000", "-ac", "1"]),
    ft!("gsm",      "GSM audio extract",              &["-vn", "-c:a", "gsm", "-ar", "8000", "-ac", "1"]),
    ft!("ircam",    "IRCAM audio extract",            &["-vn", "-c:a", "pcm_s16le"]),
    ft!("sox",      "SoX native audio extract",       &["-vn", "-c:a", "pcm_s32le"]),
];

// ── Audio input → all audio formats ──
static AUDIO_TARGETS: &[FormatTarget] = &[
    ft!("mp3",      "MP3 LAME V2",                    &["-c:a", "libmp3lame", "-q:a", "2"]),
    ft!("wav",      "WAV PCM 16-bit",                 &["-c:a", "pcm_s16le"]),
    ft!("flac",     "FLAC lossless",                  &["-c:a", "flac"]),
    ft!("ogg",      "OGG Vorbis",                     &["-c:a", "libvorbis"]),
    ft!("aac",      "AAC 192k",                       &["-c:a", "aac", "-b:a", "192k"]),
    ft!("adts",     "ADTS AAC",                       &["-c:a", "aac"]),
    ft!("opus",     "Opus",                           &["-c:a", "libopus"]),
    ft!("spx",      "Ogg Speex",                      &["-c:a", "libspeex"]),
    ft!("wma",      "WMA Windows Media Audio",        &["-c:a", "wmav2"]),
    ft!("m4a",      "M4A AAC in MP4",                 &["-c:a", "aac", "-b:a", "192k", "-movflags", "+faststart"]),
    ft!("ac3",      "AC3 Dolby 448k",                 &["-c:a", "ac3", "-b:a", "448k"]),
    ft!("eac3",     "E-AC-3 Dolby Digital+",          &["-c:a", "eac3"]),
    ft!("truehd",   "TrueHD",                         &["-c:a", "truehd"]),
    ft!("aiff",     "AIFF PCM",                       &["-c:a", "pcm_s16be"]),
    ft!("au",       "AU Sun audio",                   &["-c:a", "pcm_s16be"]),
    ft!("caf",      "CAF Core Audio Format",          &["-c:a", "pcm_s16le"]),
    ft!("voc",      "VOC Creative Voice",             &["-c:a", "pcm_s16le"]),
    ft!("w64",      "Wave64 Sony",                    &["-c:a", "pcm_s16le"]),
    ft!("tta",      "TTA True Audio",                 &["-c:a", "tta"]),
    ft!("amr",      "AMR narrowband",                 &["-c:a", "libopencore_amrnb", "-ar", "8000", "-ac", "1"]),
    ft!("gsm",      "GSM",                            &["-c:a", "gsm", "-ar", "8000", "-ac", "1"]),
    ft!("ircam",    "IRCAM",                          &["-c:a", "pcm_s16le"]),
    ft!("sox",      "SoX native",                     &["-c:a", "pcm_s32le"]),
    ft!("oga",      "OGA OGG audio",                  &["-c:a", "libvorbis"]),
];

// ── Image input → all image formats ──
static IMAGE_TARGETS: &[FormatTarget] = &[
    ft!("jpg",      "JPEG",                            &["-q:v", "2"]),
    ft!("png",      "PNG",                             &["-c:v", "png"]),
    ft!("apng",     "APNG animated PNG",               &["-c:v", "apng"]),
    ft!("webp",     "WebP",                            &["-c:v", "libwebp"]),
    ft!("avif",     "AVIF (AV1 image)",                &["-c:v", "libaom-av1", "-still-picture", "1", "-crf", "30"]),
    ft!("bmp",      "BMP bitmap",                      &[]),
    ft!("tiff",     "TIFF",                            &["-c:v", "tiff"]),
    ft!("gif",      "GIF",                             &[]),
    ft!("ico",      "ICO Windows Icon",                &[]),
    ft!("ppm",      "PPM portable pixmap",             &[]),
    ft!("pgm",      "PGM portable graymap",            &[]),
    ft!("pbm",      "PBM portable bitmap",             &[]),
    ft!("pam",      "PAM portable arbitrary map",      &[]),
    ft!("pcx",      "PCX PiCture eXchange",            &[]),
    ft!("tga",      "TGA Truevision Targa",            &[]),
    ft!("xbm",      "XBM X BitMap",                    &[]),
    ft!("xwd",      "XWD X Window Dump",               &[]),
    ft!("dpx",      "DPX Digital Picture Exchange",    &[]),
    ft!("sgi",      "SGI Silicon Graphics Image",      &[]),
    ft!("jp2",      "JP2 JPEG 2000",                   &["-c:v", "jpeg2000"]),
    ft!("hdr",      "HDR Radiance RGBE",               &[]),
    ft!("sun",      "SUN Sun Rasterfile",              &[]),
    ft!("wbmp",     "WBMP Wireless BMP",               &[]),
    ft!("pfm",      "PFM Portable FloatMap",           &[]),
    ft!("fits",     "FITS Flexible Image Transport",   &[]),
];

// ── Cross-type: Image → Video slideshow ──
static IMAGE_TO_VIDEO_TARGETS: &[FormatTarget] = &[
    ft_pre!("mp4",  "Slideshow MP4 H.264/AAC (5s)",       &["-loop", "1"], &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac", "-t", "5", "-movflags", "+faststart"]),
    ft_pre!("mkv",  "Slideshow MKV H.264/AAC (5s)",        &["-loop", "1"], &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac", "-t", "5"]),
    ft_pre!("webm", "Slideshow WebM VP8/Vorbis (5s)",      &["-loop", "1"], &["-c:v", "libvpx", "-c:a", "libvorbis", "-t", "5"]),
    ft_pre!("vp9",  "Slideshow WebM VP9/Opus (5s)", "webm", &["-loop", "1"], &["-c:v", "libvpx-vp9", "-c:a", "libopus", "-crf", "30", "-b:v", "0", "-t", "5"]),
    ft_pre!("avi",  "Slideshow AVI MPEG-4/MP3 (5s)",       &["-loop", "1"], &["-c:v", "mpeg4", "-c:a", "mp3", "-t", "5"]),
    ft_pre!("mov",  "Slideshow MOV H.264/AAC (5s)",        &["-loop", "1"], &["-c:v", "libx264", "-preset", "fast", "-c:a", "aac", "-t", "5"]),
    ft_pre!("gif",  "Animated GIF from image (2s)",        &["-loop", "1"], &["-vf", "fps=10,scale=480:-1", "-t", "2"]),
];

// ── Cross-type: Audio → Video with blank frame ──
static AUDIO_TO_VIDEO_TARGETS: &[FormatTarget] = &[
    ft_pre!("mp4",  "Audio MP4 (blank video)",    &["-f", "lavfi", "-i", "color=c=black:s=640x480:r=1", "-shortest"], &["-c:v", "libx264", "-preset", "fast", "-c:a", "copy"]),
    ft_pre!("mkv",  "Audio MKV (blank video)",    &["-f", "lavfi", "-i", "color=c=black:s=640x480:r=1", "-shortest"], &["-c:v", "libx264", "-preset", "fast", "-c:a", "copy"]),
    ft_pre!("webm", "Audio WebM (blank video)",   &["-f", "lavfi", "-i", "color=c=black:s=640x480:r=1", "-shortest"], &["-c:v", "libvpx", "-c:a", "copy"]),
    ft_pre!("avi",  "Audio AVI (blank video)",    &["-f", "lavfi", "-i", "color=c=black:s=640x480:r=1", "-shortest"], &["-c:v", "mpeg4", "-c:a", "copy"]),
    ft_pre!("mov",  "Audio MOV (blank video)",    &["-f", "lavfi", "-i", "color=c=black:s=640x480:r=1", "-shortest"], &["-c:v", "libx264", "-preset", "fast", "-c:a", "copy"]),
    ft_pre!("vp9",  "Audio WebM (blank video)", "webm", &["-f", "lavfi", "-i", "color=c=black:s=640x480:r=1", "-shortest"], &["-c:v", "libvpx-vp9", "-c:a", "copy"]),
];

pub fn get_targets(media_type: &MediaType) -> &'static [FormatTarget] {
    match media_type {
        MediaType::Video => VIDEO_TARGETS,
        MediaType::Audio => AUDIO_TARGETS,
        MediaType::Image => IMAGE_TARGETS,
    }
}

pub fn get_cross_targets(media_type: &MediaType) -> &'static [FormatTarget] {
    match media_type {
        MediaType::Image => IMAGE_TO_VIDEO_TARGETS,
        MediaType::Audio => AUDIO_TO_VIDEO_TARGETS,
        MediaType::Video => &[],
    }
}

pub fn resolve_target(media_type: &MediaType, ext: &str) -> Result<&'static FormatTarget> {
    let primary = get_targets(media_type);
    if let Some(t) = primary.iter().find(|t| t.ext == ext) {
        return Ok(t);
    }
    let cross = get_cross_targets(media_type);
    if let Some(t) = cross.iter().find(|t| t.ext == ext) {
        return Ok(t);
    }
    anyhow::bail!(
        "'{}' is not supported for {} files. Use --list to see options.",
        ext,
        media_type.label()
    );
}

pub fn print_all() {
    println!("── Supported Conversions ──\n");

    println!("══ VIDEO → video formats:");
    for t in VIDEO_TARGETS.iter().take(24) {
        let out = if t.output_ext == t.ext {
            format!(".{}", t.ext)
        } else {
            format!(".{} → .{}", t.ext, t.output_ext)
        };
        println!("  {:<14}  {}", out, t.label);
    }
    println!("\n══ VIDEO → audio extraction:");
    for t in VIDEO_TARGETS.iter().skip(24) {
        println!("  .{:<13}  {}", t.ext, t.label);
    }

    println!("\n══ AUDIO → all audio formats:");
    for t in AUDIO_TARGETS {
        println!("  .{:<13}  {}", t.ext, t.label);
    }

    println!("\n══ IMAGE → all image formats:");
    for t in IMAGE_TARGETS {
        println!("  .{:<13}  {}", t.ext, t.label);
    }

    println!("\n══ IMAGE → video slideshow:");
    for t in IMAGE_TO_VIDEO_TARGETS {
        let out = if t.output_ext == t.ext {
            format!(".{}", t.ext)
        } else {
            format!(".{} → .{}", t.ext, t.output_ext)
        };
        println!("  {:<14}  {}", out, t.label);
    }

    println!("\n══ AUDIO → video container (with blank frame):");
    for t in AUDIO_TO_VIDEO_TARGETS {
        let out = if t.output_ext == t.ext {
            format!(".{}", t.ext)
        } else {
            format!(".{} → .{}", t.ext, t.output_ext)
        };
        println!("  {:<14}  {}", out, t.label);
    }
}
