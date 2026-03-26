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

/// Built-in character sets for ASCII rendering (block, emoji, default, etc.).
pub mod charsets;

/// Webcam capture and live terminal rendering (requires the `camera` feature).
#[cfg(feature = "camera")]
pub mod camera;

mod gif_renderer;
mod image_renderer;
mod renderer;

use image::DynamicImage;
use image_renderer::ImageRenderer;
pub use renderer::RenderOptions;
use renderer::Renderer;
use std::{io, path::Path};

/// Render an image file to ASCII art, writing ANSI-colored output to `to`.
pub fn render<P: AsRef<Path> + AsRef<str>>(
    path: P,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let image = &image::open(path)?;
    render_image(image, to, options)
}

/// Render a [`DynamicImage`] to ASCII art, writing ANSI-colored output to `to`.
pub fn render_image(
    image: &DynamicImage,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let renderer = ImageRenderer::new(image, options);
    renderer.render_to(to)?;
    Ok(())
}

/// Render an image file to ASCII art, appending to a [`String`] buffer.
pub fn render_to<P: AsRef<Path> + AsRef<str>>(
    path: P,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let image = &image::open(path)?;
    let renderer = ImageRenderer::new(image, options);
    renderer.render(buffer)?;
    Ok(())
}

/// Render a [`DynamicImage`] to ASCII art, appending to a [`String`] buffer.
pub fn render_image_to(
    image: &DynamicImage,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let renderer = ImageRenderer::new(image, options);
    renderer.render(buffer)?;
    Ok(())
}
