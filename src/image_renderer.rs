use std::io;

use image::{DynamicImage, Rgba};
use owo_colors::Style;

use super::renderer::{RenderOptions, Renderer};

pub struct ImageRenderer<'a> {
    resource: &'a DynamicImage,
    options: &'a RenderOptions<'a>,
}

impl ImageRenderer<'_> {
    fn resolve_dimensions(&self) -> (u32, u32) {
        let width = self.options.width.unwrap_or_else(|| {
            (self
                .options
                .height
                .expect("Either width or height must be set") as f64
                * (self.resource.width() as f64 / self.resource.height() as f64)
                // Font cells are ~2x tall as wide
                * 2.0)
                .ceil() as u32
        });
        let height = self.options.height.unwrap_or_else(|| {
            (self
                .options
                .width
                .expect("Either width or height must be set") as f64
                * (self.resource.height() as f64 / self.resource.width() as f64)
                / 2.0)
                .ceil() as u32
        });
        (width, height)
    }

    fn get_char_for_pixel(&self, pixel: &Rgba<u8>, maximum: f64) -> &str {
        let as_grayscale = self.get_grayscale(pixel) / maximum;

        // TODO: Use alpha channel to determine if pixel is transparent?
        let char_index = (as_grayscale * (self.options.charset.len() as f64 - 1.0)) as usize;

        self.options.charset[if self.options.invert {
            self.options.charset.len() - 1 - char_index
        } else {
            char_index
        }]
    }

    fn get_grayscale(&self, pixel: &Rgba<u8>) -> f64 {
        ((pixel[0] as f64 * 0.299) + (pixel[1] as f64 * 0.587) + (pixel[2] as f64 * 0.114)) / 255.0
    }

    fn style_for_pixel(&self, pixel: &Rgba<u8>) -> Style {
        let mut style = Style::new();
        if self.options.colored {
            style = style.truecolor(pixel[0], pixel[1], pixel[2]);
        }
        if self.options.background {
            style = style.on_truecolor(pixel[0], pixel[1], pixel[2]);
        }
        style
    }
}

impl<'a> Renderer<'a, DynamicImage> for ImageRenderer<'a> {
    fn new(resource: &'a DynamicImage, options: &'a RenderOptions<'a>) -> Self {
        Self { resource, options }
    }

    fn render_to(&self, writer: &mut impl io::Write) -> io::Result<()> {
        let (width, height) = self.resolve_dimensions();
        let image = self.resource.thumbnail_exact(width, height).to_rgba8();

        let mut prev_line = 0;
        let maximum = image
            .pixels()
            .fold(0.0, |acc, pixel| self.get_grayscale(pixel).max(acc));

        for (_, line, pixel) in image.enumerate_pixels() {
            if prev_line < line {
                prev_line = line;
                writeln!(writer)?;
            }

            let style = self.style_for_pixel(pixel);
            let ch = self.get_char_for_pixel(pixel, maximum);
            write!(writer, "{}{ch}{}", style.prefix_formatter(), style.suffix_formatter())?;
        }

        writer.flush()?;
        Ok(())
    }

    fn render(&self, buffer: &mut String) -> io::Result<()> {
        let (width, height) = self.resolve_dimensions();
        let image = self.resource.thumbnail_exact(width, height).to_rgba8();

        let mut prev_line = 0;
        let maximum = image
            .pixels()
            .fold(0.0, |acc, pixel| self.get_grayscale(pixel).max(acc));

        for (_, line, pixel) in image.enumerate_pixels() {
            if prev_line < line {
                prev_line = line;
                buffer.push('\n');
            }

            let style = self.style_for_pixel(pixel);
            let ch = self.get_char_for_pixel(pixel, maximum);
            buffer.push_str(&style.prefix_formatter().to_string());
            buffer.push_str(ch);
            buffer.push_str(&style.suffix_formatter().to_string());
        }

        Ok(())
    }
}
