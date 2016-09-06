extern crate glium;

use glium::{Surface, Display, GlObject};
use common::RenderSize;
use eyebuffers::{EyeBuffers};
use programs::Programs;

pub struct Eye {
    clear_color: (f32, f32, f32, f32),
    clear_depth: f32,
    pub buffers: EyeBuffers,
    pub projection: [[f32; 4]; 4],
}

pub fn init(display: &Display,
            render_size: &RenderSize,
            projection: [[f32; 4]; 4],
            clear_color: (f32, f32, f32, f32), clear_depth: f32) -> Eye {
    Eye {
        clear_color: clear_color,
        clear_depth: clear_depth,
        buffers: EyeBuffers::new(display, render_size),
        projection: projection
    }
}

pub fn draw<T: Surface>(eye: &Eye, surface: &mut T, programs: &Programs, view_matrix: &[[f32; 4]; 4]) -> usize {
    surface.clear_color_and_depth(eye.clear_color, eye.clear_depth);
    programs.draw(surface, view_matrix, &eye.projection);
    eye.buffers.color.get_id() as usize
}