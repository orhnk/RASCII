use std::io;

#[derive(Clone, Debug)]
pub struct RenderOptions<'a> {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub colored: bool,
    pub invert: bool,
    pub charset: &'a str,
}

#[allow(dead_code)]
impl RenderOptions<'_> {
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

impl Default for RenderOptions<'_> {
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

pub trait Renderer<'a, Resource> {
    fn new(resource: Resource, options: RenderOptions<'a>) -> Self;

    fn render(&self, writer: &mut impl io::Write) -> io::Result<()>;
}
