use glium::{Display, Surface};
use floor_program::FloorProgram;
use mist_program::MistProgram;
use patch_program::PatchProgram;
use viewer::ActiveViewer;

pub struct Programs {
    floor_program: FloorProgram,
    mist_program: MistProgram,
    patch_program: PatchProgram,
}

impl Programs {
    pub fn init(display: &Display) -> Self {
        let cage = Default::default();
        Programs {
            floor_program: FloorProgram::new(display),
            mist_program: MistProgram::new(&display, &cage),
            patch_program: PatchProgram::new(&display),
        }
    }
    pub fn draw<T>(&self, viewer: &ActiveViewer, display: &Display, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) where T: Surface {
        self.mist_program.draw(surface, view, projection);
        self.patch_program.draw(viewer, display, surface, view, projection);
        self.floor_program.draw(surface, view, projection);
    }
}