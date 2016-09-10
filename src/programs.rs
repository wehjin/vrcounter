use glium::{Display, Surface};
use floor_program::FloorProgram;
use mist_program::MistProgram;
use patch_program::PatchProgram;
use controller_program::ControllerProgram;
use viewer::ActiveViewer;
use std::rc::Rc;
use std::borrow::Borrow;

pub struct Programs {
    floor_program: FloorProgram,
    mist_program: MistProgram,
    patch_program: PatchProgram,
    controller_program_option: Option<ControllerProgram>,
}

impl Programs {
    pub fn init(display: Rc<Display>, viewer: ActiveViewer, enable_controller: bool) -> Self {
        let floor_program = {
            let display_ref: &Display = display.borrow();
            FloorProgram::new(display_ref)
        };
        let mist_program = {
            let mut program = MistProgram::new(display.clone());
            program.update(vec![Default::default()]);
            program
        };
        Programs {
            floor_program: floor_program,
            mist_program: mist_program,
            patch_program: PatchProgram::new(display.clone(), viewer),
            controller_program_option: if enable_controller {
                Some(ControllerProgram::new(display.borrow()))
            } else {
                None
            }
        }
    }
    pub fn set_controller_model_matrix(&mut self, model_matrix: &Option<[[f32; 4]; 4]>) {
        if let Some(ref mut controller_program) = self.controller_program_option {
            controller_program.set_model_matrix(model_matrix);
        }
    }

    pub fn draw<T>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) where T: Surface {
        self.mist_program.draw(surface, view, projection);
        self.patch_program.draw(surface, view, projection);
        self.floor_program.draw(surface, view, projection);
        if let Some(ref controller_program) = self.controller_program_option {
            controller_program.draw(surface, view, projection);
        }
    }
}