use std::fmt;
use std::io::{self, BufWriter, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use image::DynamicImage;
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

use crate::RenderOptions;

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum CameraError {
    Permission(String),
    Device(String),
    Capture(String),
    Decode(String),
    Render(image::ImageError),
    Terminal(io::Error),
}

impl fmt::Display for CameraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Permission(msg) => write!(f, "camera permission denied: {msg}"),
            Self::Device(msg) => write!(f, "camera device error: {msg}"),
            Self::Capture(msg) => write!(f, "frame capture failed: {msg}"),
            Self::Decode(msg) => write!(f, "frame decode failed: {msg}"),
            Self::Render(err) => write!(f, "render error: {err}"),
            Self::Terminal(err) => write!(f, "terminal error: {err}"),
        }
    }
}

impl std::error::Error for CameraError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Render(err) => Some(err),
            Self::Terminal(err) => Some(err),
            _ => None,
        }
    }
}

impl From<image::ImageError> for CameraError {
    fn from(err: image::ImageError) -> Self {
        Self::Render(err)
    }
}

// ---------------------------------------------------------------------------
// CameraConfig
// ---------------------------------------------------------------------------

pub struct CameraConfig {
    pub index: u32,
    pub mirror: bool,
}

// ---------------------------------------------------------------------------
// CameraSource
// ---------------------------------------------------------------------------

pub struct CameraSource {
    camera: Camera,
    mirror: bool,
}

impl CameraSource {
    pub fn new(config: &CameraConfig) -> Result<Self, CameraError> {
        #[cfg(target_os = "macos")]
        {
            use std::sync::mpsc;
            let (tx, rx) = mpsc::channel();
            nokhwa::nokhwa_initialize(move |granted| {
                let _ = tx.send(granted);
            });
            let granted = rx.recv().map_err(|_| {
                CameraError::Permission("camera permission callback failed".into())
            })?;
            if !granted {
                return Err(CameraError::Permission("user denied camera access".into()));
            }
        }

        let mut camera = Camera::new(
            CameraIndex::Index(config.index),
            RequestedFormat::new::<RgbAFormat>(RequestedFormatType::None),
        )
        .map_err(|e| CameraError::Device(e.to_string()))?;

        camera
            .open_stream()
            .map_err(|e| CameraError::Device(e.to_string()))?;

        Ok(Self {
            camera,
            mirror: config.mirror,
        })
    }

    pub fn warmup(&mut self, frames: u32) {
        for _ in 0..frames {
            let _ = self.camera.frame();
        }
    }

    pub fn frame(&mut self) -> Result<DynamicImage, CameraError> {
        let buffer = self
            .camera
            .frame()
            .map_err(|e| CameraError::Capture(e.to_string()))?;
        let image_buf = buffer
            .decode_image::<RgbAFormat>()
            .map_err(|e| CameraError::Decode(e.to_string()))?;
        let mut image = DynamicImage::ImageRgba8(image_buf);
        if self.mirror {
            image = image.fliph();
        }
        Ok(image)
    }
}

// ---------------------------------------------------------------------------
// LiveRenderer
// ---------------------------------------------------------------------------

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show);
        let _ = terminal::disable_raw_mode();
        let _ = writeln!(io::stdout());
    }
}

pub struct LiveRenderer {
    fps: f64,
}

impl LiveRenderer {
    pub fn new(fps: f64) -> Self {
        Self { fps }
    }

    pub fn run<F>(
        &self,
        mut frame_source: F,
        options: &RenderOptions<'_>,
    ) -> Result<(), CameraError>
    where
        F: FnMut() -> Result<DynamicImage, CameraError>,
    {
        let frame_duration = Duration::from_secs_f64(1.0 / self.fps);

        terminal::enable_raw_mode().map_err(CameraError::Terminal)?;
        let _guard = TerminalGuard;

        let mut stdout = io::stdout();
        execute!(stdout, cursor::Hide, terminal::Clear(ClearType::All))
            .map_err(CameraError::Terminal)?;

        let mut frame_buf = String::new();

        loop {
            let frame_start = Instant::now();

            if event::poll(Duration::ZERO).map_err(CameraError::Terminal)? {
                if let Event::Key(key) = event::read().map_err(CameraError::Terminal)? {
                    if (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                        || key.code == KeyCode::Char('q')
                    {
                        break;
                    }
                }
            }

            match frame_source() {
                Ok(image) => {
                    frame_buf.clear();
                    crate::render_image_to(&image, &mut frame_buf, options)?;

                    let mut writer = BufWriter::new(&mut stdout);
                    execute!(writer, cursor::MoveTo(0, 0)).map_err(CameraError::Terminal)?;
                    for line in frame_buf.split('\n') {
                        write!(writer, "{}\r\n", line).map_err(CameraError::Terminal)?;
                    }
                    execute!(writer, terminal::Clear(ClearType::FromCursorDown))
                        .map_err(CameraError::Terminal)?;
                    writer.flush().map_err(CameraError::Terminal)?;
                }
                Err(_) => continue,
            }

            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                std::thread::sleep(frame_duration - elapsed);
            }
        }

        Ok(())
    }
}
