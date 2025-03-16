use std::io;

use ansi_term::Color;
use image::{DynamicImage, Rgba};

use super::renderer::{RenderOptions, Renderer};

pub struct ImageRenderer<'a> {
    resource: &'a DynamicImage,
    options: &'a RenderOptions<'a>,
}

impl ImageRenderer<'_> {
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
}

impl<'a> Renderer<'a, DynamicImage> for ImageRenderer<'a> {
    fn new(resource: &'a DynamicImage, options: &'a RenderOptions<'a>) -> Self {
        Self { resource, options }
    }

    fn render_to(&self, writer: &mut impl io::Write) -> io::Result<()> {
        let (width, height) = (
            self.options.width.unwrap_or_else(|| {
                (self
                    .options
                    .height
                    .expect("Either width or height must be set") as f64
                    * self.resource.width() as f64
                    / self.resource.height() as f64
                    // This is because the font is rarely square.
                    * 2.0)
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
                    / 2.0)
                    .ceil() as u32
            }),
        );

        let image = self.resource.thumbnail_exact(width, height).to_rgba8();

        let mut last_color: Option<Color> = None;
        let mut current_line = 0;
        let maximum = image
            .pixels()
            .fold(0.0, |acc, pixel| self.get_grayscale(pixel).max(acc));
        for (_, line, pixel) in image.enumerate_pixels() {
            if current_line < line {
                current_line = line;

                if let Some(last_color_value) = last_color {
                    write!(writer, "{}", last_color_value.suffix())?;
                    last_color = None;
                }

                writeln!(writer)?;
            }

            if self.options.colored {
                let color = Color::RGB(pixel[0], pixel[1], pixel[2]);

                if last_color != Some(color) {
                    write!(writer, "{}", color.prefix())?;
                }

                last_color = Some(color);
            }

            let char_for_pixel = self.get_char_for_pixel(pixel, maximum);
            write!(writer, "{char_for_pixel}")?;
        }

        if let Some(last_color) = last_color {
            write!(writer, "{}", last_color.suffix())?;
        }

        writer.flush()?;

        Ok(())
    }

    fn render(&self, buffer: &mut String) -> io::Result<()> {
        let (width, height) = (
            self.options.width.unwrap_or_else(|| {
                (self
                    .options
                    .height
                    .expect("Either width or height must be set") as f64
                    * self.resource.width() as f64
                    / self.resource.height() as f64
                    // This is because the font is rarely square.
                    * 2.0)
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
                    / 2.0)
                    .ceil() as u32
            }),
        );

        let image = self.resource.thumbnail_exact(width, height).to_rgba8();

        let mut last_color: Option<Color> = None;
        let mut current_line = 0;
        let maximum = image
            .pixels()
            .fold(0.0, |acc, pixel| self.get_grayscale(pixel).max(acc));
        for (_, line, pixel) in image.enumerate_pixels() {
            if current_line < line {
                current_line = line;

                if let Some(last_color_value) = last_color {
                    buffer.push_str(&last_color_value.suffix().to_string()); // TODO look up for a
                                                                             // better solution after benchmarking.
                    last_color = None;
                }

                buffer.push('\n');
            }

            if self.options.colored {
                let color = Color::RGB(pixel[0], pixel[1], pixel[2]);

                if last_color != Some(color) {
                    buffer.push_str(&color.prefix().to_string());
                }

                last_color = Some(color);
            }

            // Normally this char_for_pixel has to be a char but because of the compatibility
            // reasons with unicode-segmentation It's implemented as a &str (WORKAROUND)
            let char_for_pixel = self.get_char_for_pixel(pixel, maximum);
            buffer.push_str(char_for_pixel);
        }

        if let Some(last_color) = last_color {
            buffer.push_str(&last_color.suffix().to_string()); // TODO look up for a
                                                               // better solution after benchmarking.
        }

        Ok(())
    }
}
