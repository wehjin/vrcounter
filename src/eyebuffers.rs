extern crate glium;

use glium::framebuffer::{RenderBuffer, DepthRenderBuffer};
use glium::{Display};
use common::{RenderSize};

pub struct EyeBuffers {
    pub depth: DepthRenderBuffer,
    pub color: RenderBuffer,
}

impl EyeBuffers {
    pub fn new(display: &Display, render_size: &RenderSize) -> Self {
        let depth_buffer: DepthRenderBuffer = glium::framebuffer::DepthRenderBuffer::new(
            display,
            glium::texture::DepthFormat::I24,
            render_size.width, render_size.height).unwrap();
        let color_buffer: RenderBuffer = glium::framebuffer::RenderBuffer::new(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            render_size.width, render_size.height).unwrap();
        EyeBuffers { depth: depth_buffer, color: color_buffer }
    }
}
