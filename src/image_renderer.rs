use std::io;

use ansi_term::{Color, Style};
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

    pub fn render_grid(&self) -> crate::cell::Grid {
        let (width, height) = (
            self.options.width.unwrap_or_else(|| {
                (self
                    .options
                    .height
                    .expect("Either width or height must be set") as f64
                    * (self.resource.width() as f64 / self.resource.height() as f64)
                    * 2.0)
                    .ceil() as u32
            }),
            self.options.height.unwrap_or_else(|| {
                (self
                    .options
                    .width
                    .expect("Either width or height must be set") as f64
                    * (self.resource.height() as f64 / self.resource.width() as f64)
                    / 2.0)
                    .ceil() as u32
            }),
        );

        let image = self.resource.thumbnail_exact(width, height).to_rgba8();
        let maximum = image
            .pixels()
            .fold(0.0, |acc, pixel| self.get_grayscale(pixel).max(acc));

        let mut grid: crate::cell::Grid = Vec::new();
        let mut current_row: Vec<crate::cell::Cell> = Vec::new();
        let mut prev_line = 0;

        for (_, line, pixel) in image.enumerate_pixels() {
            if prev_line < line {
                prev_line = line;
                grid.push(current_row);
                current_row = Vec::new();
            }

            let ch = self.get_char_for_pixel(pixel, maximum).to_string();

            let (color_pre, color_suf) = if self.options.colored || self.options.background {
                let mut style = ansi_term::Style::new();
                if self.options.colored {
                    style = style.fg(ansi_term::Color::RGB(pixel[0], pixel[1], pixel[2]));
                }
                if self.options.background {
                    style = style.on(ansi_term::Color::RGB(pixel[0], pixel[1], pixel[2]));
                }
                (style.prefix().to_string(), style.suffix().to_string())
            } else {
                (String::new(), String::new())
            };

            current_row.push(crate::cell::Cell {
                ch,
                color_pre,
                color_suf,
            });
        }
        if !current_row.is_empty() {
            grid.push(current_row);
        }

        grid
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
                    * (self.resource.width() as f64 / self.resource.height() as f64)
                    // This is because the font is rarely square. TODO: Don't apply this for Chinese
                    * 2.0)
                    .ceil() as u32
            }),
            self.options.height.unwrap_or_else(|| {
                (self
                    .options
                    .width
                    .expect("Either width or height must be set") as f64
                    * (self.resource.height() as f64 / self.resource.width() as f64)
                    // This is because the font is rarely square.
                    / 2.0)
                    .ceil() as u32
            }),
        );

        let image = self.resource.thumbnail_exact(width, height).to_rgba8();

        let mut color = Style::new();
        let mut prev_line = 0;
        let maximum = image
            .pixels()
            .fold(0.0, |acc, pixel| self.get_grayscale(pixel).max(acc));
        for (_, line, pixel) in image.enumerate_pixels() {
            if self.options.colored {
                color = color.fg(Color::RGB(pixel[0], pixel[1], pixel[2]));
            }

            if self.options.background {
                color = color.on(Color::RGB(pixel[0], pixel[1], pixel[2]));
            }

            let char_for_pixel = self.get_char_for_pixel(pixel, maximum);

            if prev_line < line {
                prev_line = line;
                writeln!(writer)?;
            }

            write!(writer, "{}", color.prefix())?;
            write!(writer, "{char_for_pixel}")?;
            write!(writer, "{}", color.suffix())?;
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
                    * (self.resource.width() as f64 / self.resource.height() as f64)
                    // This is because the font is rarely square.
                    * 2.0)
                    .ceil() as u32
            }),
            self.options.height.unwrap_or_else(|| {
                (self
                    .options
                    .width
                    .expect("Either width or height must be set") as f64
                    * (self.resource.height() as f64 / self.resource.width() as f64)
                    // This is because the font is rarely square.
                    / 2.0)
                    .ceil() as u32
            }),
        );

        let image = self.resource.thumbnail_exact(width, height).to_rgba8();

        let mut color = Style::new();
        let mut prev_line = 0;
        let maximum = image
            .pixels()
            .fold(0.0, |acc, pixel| self.get_grayscale(pixel).max(acc));
        for (_, line, pixel) in image.enumerate_pixels() {
            if self.options.colored {
                color = color.fg(Color::RGB(pixel[0], pixel[1], pixel[2]));
            }

            if self.options.background {
                color = color.on(Color::RGB(pixel[0], pixel[1], pixel[2]));
            }

            if prev_line < line {
                prev_line = line;
                buffer.push('\n');
            }

            // Normally this char_for_pixel has to be a char but because of the
            // compatibility reasons with unicode-segmentation It's implemented
            // as a &str (WORKAROUND)
            let char_for_pixel = self.get_char_for_pixel(pixel, maximum);
            buffer.push_str(&color.prefix().to_string());
            buffer.push_str(char_for_pixel);
            buffer.push_str(&color.suffix().to_string());
        }

        Ok(())
    }
}
