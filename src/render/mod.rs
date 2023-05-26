mod gif;
// TODO
// pub use gif::GifRenderer as Gif;

mod image;
use std::io;

use termcolor::BufferWriter;

pub use self::image::ImageRenderer as Image;

#[derive(Debug)]
pub struct ResourceOptions {
    pub width: u32,
    pub height: u32,
    pub colorful: bool,
    pub charset: Vec<char>,
}

pub trait Renderer<Resource> {
    fn new(resource: Resource, options: ResourceOptions, writer: BufferWriter) -> Self;

    fn render(self) -> io::Result<()>;
}
