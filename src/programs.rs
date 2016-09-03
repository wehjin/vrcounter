use glium::{Display, Surface};
use floor_program::FloorProgram;
use mist_program::MistProgram;

pub struct Programs {
    floor_program: FloorProgram,
    mist_program: MistProgram,
}

impl Programs {
    pub fn new(display: &Display) -> Self {
        let cage = Default::default();
        Programs {
            floor_program: FloorProgram::new(display),
            mist_program: MistProgram::new(&display, &cage),
        }
    }
    pub fn draw<T>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) where T: Surface {
        self.floor_program.draw(surface, view, projection);
        self.mist_program.draw(surface, view, projection);
    }
}