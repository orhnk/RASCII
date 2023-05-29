use std::io;

use clap::Parser;
use rascii_art::{
    charsets,
    RenderOptions,
};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "Path to the image file.")]
    filename: String,
    #[arg(short, long, help = "Width of the output image.")]
    width: Option<u32>,
    #[arg(
        short = 'H',
        long,
        help = "Height of the output image, if not specified, it will be calculated to keep the \
                aspect ratio."
    )]
    height: Option<u32>,
    #[arg(
        name = "color",
        short,
        long,
        help = "Whether to use colors in the output image."
    )]
    colored: bool,
    #[arg(
        short,
        long,
        help = "Inverts the weights of the characters. Useful for white backgrounds."
    )]
    invert: bool,
    #[arg(
        short = 'C',
        long,
        default_value = "default",
        help = "Characters used to render the image, from translucent to opaque. Built-in \
                charsets: block, emoji, default, russian, slight."
    )]
    charset: String,
}

fn main() -> image::ImageResult<()> {
    let mut args = Args::parse();

    let clusters = UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(80);
    }

    rascii_art::render_to(
        args.filename,
        &mut io::stdout(),
        RenderOptions {
            width: args.width,
            height: args.height,
            colored: args.colored,
            invert: args.invert,
            charset,
        },
    )?;

    Ok(())
}
