use clap::Parser;

#[derive(Parser)]
#[command(
    name = "nr",
    version = "0.1.0",
    about = "Universal media converter",
    long_about = concat!(
        "Convert any media file to any compatible format.\n\n",
        "EXAMPLES:\n",
        "  nr                      clipboard file -> pick format\n",
        "  nr mp4                  clipboard file -> mp4\n",
        "  nr video.mov            detected file -> pick format\n",
        "  nr video.mov mkv        convert to mkv\n",
        "  nr --list               show all supported formats\n",
        "  nr --force mp4          batch convert all clipboard files to mp4",
    )
)]
pub struct Cli {
    /// File path or target format (auto-detected)
    pub arg1: Option<String>,

    /// Target format (only if arg1 is a file)
    pub arg2: Option<String>,

    /// Show interactive conversion menu
    #[arg(long)]
    pub convert: bool,

    /// Show all supported formats
    #[arg(long, short)]
    pub list: bool,

    /// Batch convert all clipboard files to FORMAT (e.g. --force mp4)
    #[arg(long)]
    pub force: Option<String>,
}

/// Known format extensions
pub fn is_format_name(s: &str) -> bool {
    matches!(
        s,
        "mp4" | "h265" | "mkv" | "webm" | "vp9" | "avi" | "mov"
            | "wmv" | "flv" | "m4v" | "ogv" | "mpg" | "mpeg" | "ts"
            | "3gp" | "3g2" | "gif" | "dv" | "m2ts" | "vob" | "av1"
            | "mxf" | "dnxhd" | "mjpeg" | "nut" | "psp"
            | "mp3" | "wav" | "flac" | "ogg" | "aac" | "adts" | "opus"
            | "spx" | "wma" | "m4a" | "ac3" | "eac3" | "truehd"
            | "aiff" | "au" | "caf" | "voc" | "w64" | "tta" | "amr"
            | "gsm" | "ircam" | "sox" | "oga"
            | "jpg" | "jpeg" | "png" | "apng" | "bmp" | "tiff" | "tif"
            | "webp" | "avif" | "ppm" | "pgm" | "pbm" | "pam" | "pcx"
            | "tga" | "xbm" | "xwd" | "dpx" | "sgi" | "jp2" | "hdr"
            | "sun" | "wbmp" | "pfm" | "ico" | "fits" | "heic" | "heif"
    )
}
