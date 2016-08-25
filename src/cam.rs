use mat;

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

    pub fn up(&self) -> Camera {
        Camera { eye: self.eye, look: mat::add_y(&self.look, STEP), up: self.up }
    }

    pub fn down(&self) -> Camera {
        Camera { eye: self.eye, look: mat::add_y(&self.look, -STEP), up: self.up }
    }

    pub fn left(&self) -> Camera {
        Camera { eye: self.eye, look: mat::add_x(&self.look, -STEP), up: self.up }
    }

    pub fn right(&self) -> Camera {
        Camera { eye: self.eye, look: mat::add_x(&self.look, STEP), up: self.up }
    }

    pub fn near(&self) -> Camera {
        Camera { eye: mat::add_z(&self.eye, -STEP), look: self.look, up: self.up }
    }

    pub fn far(&self) -> Camera {
        Camera { eye: mat::add_z(&self.eye, STEP), look: self.look, up: self.up }
    }
}