pub struct Camera {
    pub eye: [f32; 3],
    pub look: [f32; 3],
    pub up: [f32; 3],
}

impl Camera {
    pub fn make() -> Camera {
        Camera { eye: [0.0, 1.6, 0.0], look: [0.0, 0.0, -1.0], up: [0.0, 1.0, 0.0] }
    }
}