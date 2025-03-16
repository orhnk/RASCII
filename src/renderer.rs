use std::io;

use crate::charsets;

#[derive(Clone, Debug)]
pub struct RenderOptions<'a> {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub colored: bool,
    pub escape_each_colored_char: bool,
    pub invert: bool,
    pub charset: &'a [&'a str],
}

#[allow(dead_code)]
impl<'a> RenderOptions<'a> {
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

    /// Set whether each colored char should be escaped in the rendered image.
    pub fn escape_each_colored_char(mut self, escape_each_colored_char: bool) -> Self {
        self.escape_each_colored_char = escape_each_colored_char;
        self
    }

    /// Set whether the rendered image charset should be inverted.
    pub fn invert(mut self, invert: bool) -> Self {
        self.invert = invert;
        self
    }

    /// Set the charset to use for the rendered image.
    pub fn charset(mut self, charset: &'a [&'a str]) -> Self {
        self.charset = charset;
        self
    }
}

impl Default for RenderOptions<'_> {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            colored: false,
            escape_each_colored_char: false,
            invert: false,
            charset: charsets::DEFAULT,
        }
    }
}

pub trait Renderer<'a, R> {
    fn new(resource: &'a R, options: &'a RenderOptions<'a>) -> Self;

    fn render_to(&self, writer: &mut impl io::Write) -> io::Result<()>;

    fn render(&self, writer: &mut String) -> io::Result<()>;
}
