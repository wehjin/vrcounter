use floor_program::FloorProgram;
use glium::{Display, Surface};

pub struct Programs {
    floor_program: FloorProgram,
}

impl Programs {
    pub fn new(display: &Display) -> Self {
        Programs {
            floor_program: FloorProgram::new(display)
        }
    }
    pub fn draw<T>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) where T: Surface {
        self.floor_program.draw(surface, view, projection);
    }
}