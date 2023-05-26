mod render;
use clap::Parser;
use render::Renderer;
use termcolor::{
    BufferWriter,
    ColorChoice,
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "Path to the image file.")]
    filename: String,
    #[arg(short, long, default_value = "128", help = "Width of the output image")]
    width: u32,
    #[arg(
        short = 'H',
        long,
        default_value = "0",
        help = "Height of the output image, if not specified, it will be calculated to keep the \
                aspect ratio."
    )]
    height: u32,
    #[arg(
        short,
        long,
        help = "Whether to use colors in the output image."
    )]
    color: bool,
    #[arg(
        short = 'C',
        long,
        default_value = " .,:;i1tfLCG08@",
        help = "Characters used to render the image, from translucent to opaque."
    )]
    charset: String,
}

fn main() -> image::ImageResult<()> {
    let mut args = Args::parse();

    let dyn_image = image::open(args.filename)?;

    if args.height == 0 {
        // The 2.0 multiplier is because terminals often have a 1 to 2 aspect ratio on the width and height.
        args.height = (dyn_image.height() as f64 * args.width as f64 / dyn_image.width() as f64 / 2.0) as u32;
    }

    let options = render::ResourceOptions {
        width: args.width,
        height: args.height,
        colorful: args.color,
        charset: args.charset.chars().collect(),
    };

    let buffer_writer = BufferWriter::stdout(ColorChoice::Always);
    render::Image::new(dyn_image, options, buffer_writer).render()?;

    Ok(())
}
