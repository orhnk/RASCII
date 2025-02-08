use std::io;

use clap::Parser;
use rascii_art::{charsets, RenderOptions};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to the image
    filename: String,

    /// Width of the output image. Defaults to 128 if width and height are not
    /// specified
    #[arg(short, long)]
    width: Option<u32>,

    /// Height of the output image, if not specified, it will be calculated to
    /// keep the aspect ratio
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Whether to use colors in the output image
    #[arg(name = "color", short, long)]
    colored: bool,

    /// Whether all characters should have an ANSI color code before each character.
    /// Defaults to only escape colored strings upon color change.
    #[arg(name = "escape-each-char", short, long)]
    escape_each_colored_char: bool,

    /// Inverts the weights of the characters. Useful for white backgrounds
    #[arg(short, long)]
    invert: bool,

    /// Characters used to render the image, from transparent to opaque.
    /// Built-in charsets: block, emoji, default, russian, slight
    #[arg(short = 'C', long, default_value = "default")]
    charset: String,
}

fn main() -> image::ImageResult<()> {
    let mut args = Args::parse();

    let clusters = UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(80);
    }

    rascii_art::render(
        &args.filename,
        &mut io::stdout(),
        &RenderOptions {
            width: args.width,
            height: args.height,
            colored: args.colored,
            invert: args.invert,
            escape_each_colored_char: args.escape_each_colored_char,
            charset,
        },
    )?;

    Ok(())
}
