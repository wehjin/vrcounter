use glium::{Display, Surface};
use floor_program::FloorProgram;
use mist_program::MistProgram;
use patch_program::PatchProgram;
use viewer::ActiveViewer;
use std::rc::Rc;
use std::borrow::Borrow;

pub struct Programs {
    floor_program: FloorProgram,
    mist_program: MistProgram,
    patch_program: PatchProgram,
}

impl Programs {
    pub fn init(display: Rc<Display>, viewer: ActiveViewer) -> Self {
        let floor_program = {
            let display_ref: &Display = display.borrow();
            FloorProgram::new(display_ref)
        };
        let mist_program = {
            let display_ref: &Display = display.borrow();
            let cage = Default::default();
            MistProgram::new(display_ref, &cage)
        };
        Programs {
            floor_program: floor_program,
            mist_program: mist_program,
            patch_program: PatchProgram::new(display, viewer),
        }
    }
    pub fn draw<T>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) where T: Surface {
        self.mist_program.draw(surface, view, projection);
        self.patch_program.draw(surface, view, projection);
        self.floor_program.draw(surface, view, projection);
    }
}