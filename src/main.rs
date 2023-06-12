use std::io;

use clap::Parser;
use rascii_art::{
    Charset,
    RenderOptions,
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "Path to the image")]
    filename: String,
    #[arg(
        short,
        long,
        help = "Width of the output image. Defaults to 128 if width and height are not specified"
    )]
    width: Option<u32>,
    #[arg(
        short = 'H',
        long,
        help = "Height of the output image, if not specified, it will be calculated to keep the \
                aspect ratio"
    )]
    height: Option<u32>,
    #[arg(
        name = "color",
        short,
        long,
        help = "Whether to use colors in the output image"
    )]
    colored: bool,
    #[arg(
        short,
        long,
        help = "Inverts the weights of the characters. Useful for white backgrounds"
    )]
    invert: bool,
    #[arg(
        short = 'C',
        long,
        default_value = "default",
        help = "Characters used to render the image, from transparent to opaque. Built-in \
                charsets: block, emoji, default, russian, slight"
    )]
    charset: String,
}

fn main() -> image::ImageResult<()> {
    let mut args = Args::parse();

    let charset: Charset = args.charset.as_str().into();

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
