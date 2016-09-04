use mat;
use std::f32::consts::PI;
use glium::{Surface};

static STEP: f32 = 0.04;

pub struct Camera {
    pub eye: [f32; 3],
    pub look: [f32; 3],
    pub up: [f32; 3],
}

impl Camera {
    pub fn start() -> Camera {
        Camera { eye: [0.0, 1.6, 0.0], look: [0.0, 0.0, -1.0], up: [0.0, 1.0, 0.0] }
    }

    pub fn get_view_and_projection<T>(&self, surface: &T) -> ([[f32; 4]; 4], [[f32; 4]; 4]) where T: Surface {
        let view = mat::view_matrix(&self.eye, &self.look, &self.up);
        let perspective = mat::perspective_matrix(surface.get_dimensions(), PI / 3.0);
        (view, perspective)
    }

    pub fn move_up(&self) -> Camera {
        // TODO: Fix these to rotate look.
        Camera { eye: self.eye, look: mat::add_y(&self.look, STEP), up: self.up }
    }

    pub fn move_down(&self) -> Camera {
        Camera { eye: self.eye, look: mat::add_y(&self.look, -STEP), up: self.up }
    }

    pub fn move_left(&self) -> Camera {
        Camera { eye: self.eye, look: mat::add_x(&self.look, -STEP), up: self.up }
    }

    pub fn move_right(&self) -> Camera {
        Camera { eye: self.eye, look: mat::add_x(&self.look, STEP), up: self.up }
    }

    pub fn move_near(&self) -> Camera {
        // TODO: Fix these to move forward along look.
        Camera { eye: mat::add_z(&self.eye, -STEP), look: self.look, up: self.up }
    }

    pub fn move_far(&self) -> Camera {
        Camera { eye: mat::add_z(&self.eye, STEP), look: self.look, up: self.up }
    }
}