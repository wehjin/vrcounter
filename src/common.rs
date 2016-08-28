extern crate openvr;

#[derive(Debug)]
pub enum Error {
    NoSystem,
    NoCompositor,
}

#[derive(Debug)]
pub struct RenderSize {
    pub width: u32,
    pub height: u32
}

impl From<openvr::common::Size> for RenderSize {
    fn from(size: openvr::common::Size) -> Self {
        RenderSize { width: size.width, height: size.height }
    }
}
