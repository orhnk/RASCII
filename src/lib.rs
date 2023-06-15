pub mod charsets;
pub mod craiyon;

pub use renderer::RenderOptions;

mod gif_renderer;
mod image_renderer;
mod renderer;

use renderer::Renderer;
use std::{io, path::Path};

pub fn render_to<P: AsRef<Path>>(
    path: P,
    to: &mut impl io::Write,
    options: RenderOptions<'_>,
) -> image::ImageResult<()> {
    render_image_to(image::open(path)?, to, options)
}

pub fn render_image_to(
    image: image::DynamicImage,
    to: &mut impl io::Write,
    options: RenderOptions<'_>,
) -> image::ImageResult<()> {
    let renderer = image_renderer::ImageRenderer::new(image, options);
    renderer.render(to)?;
    Ok(())
}
