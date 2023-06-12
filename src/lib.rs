mod charset;
mod gif_renderer;
// pub use gif_renderer::GifRenderer;
mod image_renderer;
use image_renderer::ImageRenderer;
mod renderer;
use renderer::Renderer;
use std::{io, path::Path};

pub use charset::Charset;
pub use renderer::RenderOptions;

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
    let renderer = ImageRenderer::new(image, options);
    renderer.render(to)?;
    Ok(())
}
