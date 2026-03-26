use std::io;

use clap::Parser;
use rascii_art::{charsets, RenderOptions};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to the image
    filename: Option<String>,

    /// Use webcam as input
    #[cfg(feature = "camera")]
    #[arg(long, conflicts_with = "filename")]
    camera: bool,

    /// Continuous live rendering (use with --camera)
    #[cfg(feature = "camera")]
    #[arg(long, requires = "camera")]
    live: bool,

    /// Frames per second for live mode
    #[cfg(feature = "camera")]
    #[arg(long, default_value_t = 4.0, requires = "live")]
    fps: f64,

    /// Camera device index
    #[cfg(feature = "camera")]
    #[arg(long, default_value_t = 0)]
    camera_index: u32,

    /// Disable selfie mirror (horizontal flip)
    #[cfg(feature = "camera")]
    #[arg(long)]
    no_mirror: bool,

    /// Number of warmup frames for auto-exposure
    #[cfg(feature = "camera")]
    #[arg(long, default_value_t = 30)]
    warmup: u32,

    /// Width of the output image. Defaults to terminal width
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

    /// Characters used to render the image, from transparent to opaque.
    /// Built-in charsets: block, emoji, default, russian, slight
    #[arg(short = 'C', long, default_value = "default")]
    charset: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();

    let clusters =
        UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

    if args.width.is_none() && args.height.is_none() {
        let (cols, _) = crossterm::terminal::size().unwrap_or((80, 24));
        args.width = Some(cols as u32);
    }

    let options = RenderOptions {
        width: args.width,
        height: args.height,
        colored: args.colored,
        background: args.background,
        invert: args.invert,
        charset,
    };

    #[cfg(feature = "camera")]
    if args.camera {
        use rascii_art::camera::{CameraConfig, CameraSource, LiveRenderer};

        let config = CameraConfig {
            index: args.camera_index,
            mirror: !args.no_mirror,
        };
        let mut source = CameraSource::new(&config)?;
        source.warmup(args.warmup);

        if args.live {
            LiveRenderer::new(args.fps).run(|| source.frame(), &options)?;
        } else {
            let image = source.frame()?;
            rascii_art::render_image(&image, &mut io::stdout(), &options)?;
        }
        return Ok(());
    }

    if let Some(ref filename) = args.filename {
        rascii_art::render(filename, &mut io::stdout(), &options)?;
    } else {
        eprintln!("Error: provide a filename or use --camera");
        std::process::exit(1);
    }

    Ok(())
}
