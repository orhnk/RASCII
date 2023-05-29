mod gif;
mod image;

use std::fmt;

pub use self::image::ImageRenderer;
// TODO: pub use self::gif::GifRenderer;

#[derive(Clone, Debug)]
pub struct RenderOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub colored: bool,
    pub invert: bool,
    pub charset: &'static str,
}

impl RenderOptions {
    /// Create a new RenderOptions with default values.
    pub fn new() -> Self {
        Self::default()
    }

    // BUILDER METHODS

    /// Set the width of the rendered image.
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the rendered image.
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set whether the rendered image should be colored.
    pub fn colored(mut self, colorful: bool) -> Self {
        self.colored = colorful;
        self
    }

    /// Set whether the rendered image charset should be inverted.
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    /// Set the charset to use for the rendered image.
    pub fn charset(mut self, charset: &'static str) -> Self {
        self.charset = charset;
        self
    }
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            width: Some(128),
            height: None,
            colored: false,
            invert: false,
            charset: crate::charsets::DEFAULT,
        }
    }
}

pub trait Renderer<Resource> {
    fn new(resource: Resource, options: RenderOptions) -> Self;

    fn render(&self, writer: &mut impl fmt::Write) -> fmt::Result;
}
