use std::io::{
    self,
    Write,
};

use image::{
    DynamicImage,
    Rgba,
};
use termcolor::{
    Buffer,
    BufferWriter,
    Color,
    ColorSpec,
    WriteColor,
};

use super::{
    Renderer,
    ResourceOptions,
};

pub struct ImageRenderer {
    image: DynamicImage,
    options: ResourceOptions,
    writer: BufferWriter,
}

impl ImageRenderer {
    fn set_color(&mut self, buffer: &mut Buffer, pixel: &Rgba<u8>) -> io::Result<()> {
        let color = Color::Rgb(pixel[0], pixel[1], pixel[2]);
        buffer.set_color(ColorSpec::new().set_fg(Some(color)))
    }

    fn get_char_for_pixel(&self, pixel: &Rgba<u8>) -> char {
        let as_grayscale = (pixel[0] as f64 * 0.299) + (pixel[1] as f64 * 0.587) + (pixel[2] as f64 * 0.114);
        
        // TODO: Use alpha channel to determine if pixel is transparent?
        let char_index =
            ((as_grayscale / 255.0) * (self.options.charset.len() as f64 - 1.0)) as usize;

        if self.options.invert {
            self.options.charset[self.options.charset.len() - 1 - char_index]
        } else {
            self.options.charset[char_index]
        }
    }
}

impl Renderer<DynamicImage> for ImageRenderer {
    fn new(image: DynamicImage, options: ResourceOptions, writer: BufferWriter) -> Self {
        Self {
            image,
            options,
            writer,
        }
    }

    fn render(mut self) -> io::Result<()> {
        let mut buffer = self.writer.buffer();
        let image = self
            .image
            .thumbnail_exact(self.options.width, self.options.height)
            .to_rgba8();

        let mut current_line = 0;
        for (_, line, pixel) in image.enumerate_pixels() {
            if current_line < line {
                current_line = line;
                writeln!(&mut buffer)?;
            }

            if self.options.colorful {
                self.set_color(&mut buffer, pixel)?;
            }

            let char_for_pixel = self.get_char_for_pixel(pixel);
            write!(&mut buffer, "{}", char_for_pixel)?;
        }

        buffer.set_color(&Default::default())?;
        self.writer.print(&buffer)?;

        Ok(())
    }
}
