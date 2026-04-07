use std::fs::File;
use std::io::{self, BufReader, Write};
use std::thread;
use std::time::Duration;

use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage};

use crate::cell::Grid;
use crate::image_renderer::ImageRenderer;
use crate::renderer::{RenderOptions, Renderer};

pub struct GifRenderer;

impl GifRenderer {
    pub fn play(path: &str, options: &RenderOptions<'_>) -> io::Result<()> {
        let file = File::open(path).map_err(io::Error::other)?;
        let decoder = GifDecoder::new(BufReader::new(file)).map_err(io::Error::other)?;
        let frames: Vec<_> = decoder
            .into_frames()
            .collect::<Result<Vec<_>, _>>()
            .map_err(io::Error::other)?;

        if frames.is_empty() {
            return Ok(());
        }

        let mut out = io::stdout();
        write!(out, "\x1b[?25l\x1b[2J")?;
        out.flush()?;

        // Pre-render all frames to grids
        let grids: Vec<(Grid, Duration)> = frames
            .iter()
            .map(|frame| {
                let (numer, denom) = frame.delay().numer_denom_ms();
                let ms = (numer as f64 / denom.max(1) as f64).max(1.0);
                let delay = Duration::from_millis(ms.round() as u64);
                let image = DynamicImage::ImageRgba8(frame.buffer().clone());
                let image = if options.trim {
                    crate::trim::trim_image(&image)
                } else {
                    image
                };
                let renderer = ImageRenderer::new(&image, options);
                let grid = renderer.render_grid();
                (grid, delay)
            })
            .collect();

        // Play loop — Ctrl-C to stop
        loop {
            for (grid, delay) in &grids {
                write!(out, "\x1b[H")?;
                for (r, row) in grid.iter().enumerate() {
                    for cell in row {
                        write!(out, "{}{}{}", cell.color_pre, cell.ch, cell.color_suf)?;
                    }
                    if r < grid.len() - 1 {
                        writeln!(out)?;
                    }
                }
                out.flush()?;
                thread::sleep(*delay);
            }
        }
    }
}
