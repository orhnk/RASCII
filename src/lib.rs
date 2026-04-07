//! # Usage:
//! ```no_run
//! use rascii_art::{
//!     render_to,
//!     RenderOptions,
//! };
//!
//! fn main() {
//!     let mut buf = String::new();
//!
//!     render_to(
//!         r"/path/to/image.png",
//!         &mut buf,
//!         &RenderOptions::new()
//!             .width(100)
//!             .colored(true)
//!             .charset(&[".", ",", "-", "*", "£", "$", "#"]),
//!     )
//!     .unwrap();
//! }
//! ```

pub mod animator;
pub mod cell;
pub mod charsets;

pub(crate) mod gif_renderer;
pub(crate) mod image_renderer;
mod renderer;
pub(crate) mod trim;

use image::DynamicImage;
use image_renderer::ImageRenderer;
pub use renderer::RenderOptions;
use renderer::Renderer;
use std::{io, path::Path};

pub fn render_gif(path: &str, options: &RenderOptions<'_>) -> io::Result<()> {
    gif_renderer::GifRenderer::play(path, options)
}

pub use trim::trim_image;

pub fn render<P: AsRef<Path> + AsRef<str>>(
    path: P,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let image = &image::open(path)?;
    render_image(image, to, options)
}

pub fn render_image(
    image: &DynamicImage,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let owned;
    let img = if options.trim {
        owned = trim::trim_image(image);
        &owned
    } else {
        image
    };
    let renderer = ImageRenderer::new(img, options);
    renderer.render_to(to)?;
    Ok(())
}

pub fn render_to<P: AsRef<Path> + AsRef<str>>(
    path: P,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let image = image::open(path)?;
    let image = if options.trim {
        trim::trim_image(&image)
    } else {
        image
    };
    let renderer = ImageRenderer::new(&image, options);
    renderer.render(buffer)?;
    Ok(())
}

pub fn render_grid(image: &DynamicImage, options: &RenderOptions<'_>) -> cell::Grid {
    let renderer = ImageRenderer::new(image, options);
    renderer.render_grid()
}

pub fn render_image_to(
    image: &DynamicImage,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let owned;
    let img = if options.trim {
        owned = trim::trim_image(image);
        &owned
    } else {
        image
    };
    let renderer = ImageRenderer::new(img, options);
    renderer.render(buffer)?;
    Ok(())
}
