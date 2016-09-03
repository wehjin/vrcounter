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

#[derive(Debug)]
pub struct IdSource {
    global_id: u64,
}

impl IdSource {
    pub fn new() -> Self {
        IdSource { global_id: 1u64 }
    }
    pub fn next_id(&mut self) -> u64 {
        let id = self.global_id;
        self.global_id = self.global_id + 1;
        id
    }
}
