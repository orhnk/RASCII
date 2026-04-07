use std::io;

use clap::Parser;
use rascii_art::{animator, charsets, RenderOptions};
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

    /// Highlight background
    #[arg(short = 'b', long)]
    background: bool,

    /// Inverts the weights of the characters. Useful for white backgrounds
    #[arg(short, long)]
    invert: bool,

    /// Trim empty borders (transparent, solid color) from the image
    #[arg(short = 't', long)]
    trim: bool,

    /// Characters used to render the image, from transparent to opaque.
    /// Built-in charsets: block, blocks, braille, chinese, default, dense,
    /// emoji, hybrid, russian, slight, stipple
    #[arg(short = 'C', long, default_value = "default")]
    charset: String,

    /// Animate the output with a terminal effect.
    /// Effects: dissolve-in, dissolve-out, swirl-in, swirl-out, whirl-in,
    /// whirl-out, ants-in, ants-out, ken-burns
    #[arg(short = 'a', long)]
    animate: Option<String>,

    /// Duration of the animation in seconds
    #[arg(short = 'd', long, default_value = "3.0")]
    duration: f64,
}

fn main() -> image::ImageResult<()> {
    let mut args = Args::parse();

    let clusters = UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(80);
    }

    let options = RenderOptions {
        width: args.width,
        height: args.height,
        colored: args.colored,
        background: args.background,
        invert: args.invert,
        trim: args.trim,
        charset,
    };

    // Check if input is an animated GIF
    if args.filename.to_lowercase().ends_with(".gif") {
        rascii_art::render_gif(&args.filename, &options).map_err(image::ImageError::IoError)?;
        return Ok(());
    }

    // Check for animation effect
    if let Some(ref effect_name) = args.animate {
        let effect = animator::Effect::from_str(effect_name).unwrap_or_else(|| {
            eprintln!(
                "Unknown animation effect: {}. Valid: dissolve-in, dissolve-out, swirl-in, swirl-out, whirl-in, whirl-out, ants-in, ants-out, ken-burns",
                effect_name
            );
            std::process::exit(1);
        });

        let image = image::open(&args.filename)?;
        let image = if args.trim {
            rascii_art::trim_image(&image)
        } else {
            image
        };
        let grid = rascii_art::render_grid(&image, &options);
        let anim = animator::Animator::new(grid, effect, args.duration);
        anim.play().map_err(image::ImageError::IoError)?;
        return Ok(());
    }

    // Static render
    rascii_art::render(&args.filename, &mut io::stdout(), &options)?;

    Ok(())
}
