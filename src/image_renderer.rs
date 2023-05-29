use std::io;

use ansi_term::Color;
use image::{
    DynamicImage,
    Rgba,
};

use super::{
    RenderOptions,
    Renderer,
};

pub struct ImageRenderer<'a> {
    resource: DynamicImage,
    options: RenderOptions<'a>,
    last_color: Option<Color>,
}

impl ImageRenderer<'_> {
    fn get_char_for_pixel(&self, pixel: &Rgba<u8>) -> &str {
        let as_grayscale =
            (pixel[0] as f64 * 0.299) + (pixel[1] as f64 * 0.587) + (pixel[2] as f64 * 0.114);

        // TODO: Use alpha channel to determine if pixel is transparent?
        let char_index =
            ((as_grayscale / 255.0) * (self.options.charset.len() as f64 - 1.0)) as usize;

        self.options.charset[if self.options.invert {
            self.options.charset.len() - 1 - char_index
        } else {
            char_index
        }]
    }
}

impl<'a> Renderer<'a, DynamicImage> for ImageRenderer<'a> {
    fn new(resource: DynamicImage, options: RenderOptions<'a>) -> Self {
        Self {
            resource,
            options,
            last_color: None,
        }
    }

    fn render(&self, writer: &mut impl io::Write) -> io::Result<()> {
        let (width, height) = (
            self.options.width.unwrap_or_else(|| {
                (self
                    .options
                    .height
                    .expect("Either width or height must be set") as f64
                    * self.resource.width() as f64
                    / self.resource.height() as f64)
                    .ceil() as u32
            }),
            self.options.height.unwrap_or_else(|| {
                (self
                    .options
                    .width
                    .expect("Either width or height must be set") as f64
                    * self.resource.height() as f64
                    / self.resource.width() as f64
                    // This is because the font is rarely square.
                    * 2.0)
                    .ceil() as u32
            }),
        );

        let image = self.resource.thumbnail_exact(width, height).to_rgba8();

        let mut current_line = 0;

        for (_, line, pixel) in image.enumerate_pixels() {
            if current_line < line {
                current_line = line;

                if let Some(last_color) = self.last_color {
                    write!(writer, "{}", last_color.suffix())?;
                }

                writeln!(writer)?;
            }

            if self.options.colored {
                let color = Color::RGB(pixel[0], pixel[1], pixel[2]);

                if self.last_color != Some(color) {
                    write!(writer, "{}", color.prefix())?;
                }
            }

            let char_for_pixel = self.get_char_for_pixel(pixel);
            write!(writer, "{char_for_pixel}")?;
        }

        if let Some(last_color) = self.last_color {
            write!(writer, "{}", last_color.suffix())?;
        }

        writer.flush()?;

        Ok(())
    }
}
