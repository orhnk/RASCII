mod charsets;

mod gif_renderer;
// use gif_renderer::GifRenderer;

mod image_renderer;
use image_renderer::ImageRenderer;

mod renderer;
use clap::Parser;
use renderer::{
    RenderOptions,
    Renderer,
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "Path to the image file.")]
    filename: String,
    #[arg(short, long, default_value = "128", help = "Width of the output image")]
    width: Option<u32>,
    #[arg(
        short = 'H',
        long,
        default_value = "0",
        help = "Height of the output image, if not specified, it will be calculated to keep the \
                aspect ratio."
    )]
    height: Option<u32>,
    #[arg(name = "color", short, long, help = "Whether to use colors in the output image.")]
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
    let args = Args::parse();

    let dyn_image = image::open(args.filename)?;

    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(args.charset.as_str());

    ImageRenderer::new(
        dyn_image,
        RenderOptions {
            width: args.width,
            height: args.height,
            colored: args.colored,
            invert: args.invert,
            charset,
        },
    )
    .render(todo!())?;

    Ok(())
}
