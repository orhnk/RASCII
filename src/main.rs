use clap::Parser;
use rascii_art::{
    charsets,
    craiyon::{Api, Model, ModelType},
    RenderOptions,
};
use spinners::{Spinner, Spinners};
use std::io;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(group = "source", help = "Path to the image")]
    filename: Option<String>,

    #[arg(short, long, group = "source", help = "Use AI to generate ascii art")]
    query: Option<String>,

    #[arg(
        short,
        long,
        default_value = "",
        requires = "query",
        conflicts_with = "filename",
        help = "Use AI to generate ascii art, but with a negative query"
    )]
    negative_query: Option<String>,

    #[arg(
        short = 'N',
        long,
        value_name = "NUMBER",
        default_value = "9",
        requires = "query",
        conflicts_with = "filename",
        help = "Number of images to generate when using AI [1..9]"
    )]
    num_image: usize,

    #[arg(
        value_enum,
        short,
        long,
        default_value = "general",
        requires = "query",
        conflicts_with = "filename",
        help = "Model to use in generation"
    )]
    model_type: Option<ModelType>,

    #[arg(
        value_enum,
        short,
        name = "API_VERSION",
        default_value = "3",
        requires = "query",
        conflicts_with = "filename",
        help = "Model API version"
    )]
    version: Option<Api>,

    #[arg(
        short,
        long,
        name = "TOKEN",
        requires = "query",
        conflicts_with = "filename",
        help = "API token for premium users (Faster generation, No watermark)"
    )]
    api_token: Option<String>,

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

// TODO
// stderr for non ascii art

#[tokio::main]
async fn main() -> image::ImageResult<()> {
    let mut args = Args::parse();

    let clusters = UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(80);
    }

    if args.query.is_some() {
        let query = args.query.unwrap();
        let mut sp = Spinner::new(Spinners::Arc, query.clone()); // FIXME

        let model = Model::from(args.model_type.unwrap(), args.version.unwrap()).api_token(args.api_token.as_deref());
        let images = model
            .generate(&query, &args.negative_query.unwrap(), args.num_image)
            .await;

        sp.stop_with_symbol("\x1b[32m✔\x1b[0m");

        for image in images {
            rascii_art::render_image_to(
                image,
                &mut io::stdout(),
                RenderOptions {
                    width: args.width,
                    height: args.height,
                    colored: args.colored,
                    invert: args.invert,
                    charset,
                },
            )
            .expect("Failed to generate image");
            println!("\n");
        }

        return Ok(());
    }

    rascii_art::render_to(
        args.filename.unwrap(),
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
