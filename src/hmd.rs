extern crate glium;

use glium::backend::glutin_backend::GlutinFacade;
use vr::System;
use eye;
use eye::Eye;
use constants::{CLEAR_COLOR, CLEAR_DEPTH};
use programs::Programs;
use glium::Surface;

pub struct Hmd<'a> {
    vr: &'a System,
    pub left_eye: Eye,
    pub right_eye: Eye,
}

impl<'a> Hmd<'a> {
    pub fn new(window: &GlutinFacade, vr: &'a System) -> Self {
        let render_size = vr.get_render_size();
        let (left_eye, right_eye) = (
            eye::init(window, &render_size, vr.get_left_projection(), CLEAR_COLOR, CLEAR_DEPTH),
            eye::init(window, &render_size, vr.get_right_projection(), CLEAR_COLOR, CLEAR_DEPTH)
        );
        Hmd { vr: vr, left_eye: left_eye, right_eye: right_eye }
    }

    pub fn draw<T: Surface>(&self, programs: &Programs, view_matrix: &[[f32; 4]; 4],
                            window: &GlutinFacade, left_surface: &mut T, right_surface: &mut T) {
        let mut target = window.draw();
        target.clear_color_and_depth(CLEAR_COLOR, CLEAR_DEPTH);
        programs.draw(&mut target, view_matrix, &self.left_eye.projection);
        target.finish().unwrap();

        let (left_texture, right_texture) = (
            eye::draw(&self.left_eye, left_surface, programs, view_matrix),
            eye::draw(&self.right_eye, right_surface, programs, view_matrix)
        );
        self.vr.submit_textures(left_texture, right_texture);
    }
}