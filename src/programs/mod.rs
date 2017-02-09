mod floor_program;
mod mist_program;
mod hand_program;
mod patch_program;

use glium::{Display, Surface};
use programs::floor_program::FloorProgram;
use programs::mist_program::MistProgram;
use programs::patch_program::PatchProgram;
use programs::hand_program::HandProgram;
use controller_program::ControllerProgram;
use viewer::Viewer;
use std::rc::Rc;
use std::borrow::Borrow;

pub struct Programs {
    floor_program: FloorProgram,
    mist_program: MistProgram,
    patch_program: PatchProgram,
    controller_program_option: Option<ControllerProgram>,
    hand_program_option: Option<HandProgram>
}

impl Programs {
    pub fn new(display: Rc<Display>, viewer: Viewer, hand_type: HandType) -> Self {
        Programs {
            floor_program: FloorProgram::new(display.clone()),
            mist_program: MistProgram::new(display.clone(), viewer.clone()),
            patch_program: PatchProgram::new(display.clone(), viewer.clone()),
            controller_program_option: match hand_type {
                HandType::Vive => Some(ControllerProgram::new(display.borrow())),
                _ => None
            },
            hand_program_option: match hand_type {
                HandType::Keyboard => Some(HandProgram::new(display.clone(), viewer.clone())),
                _ => None
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
        if let Some(ref hand_program) = self.hand_program_option {
            hand_program.draw(surface, view, projection);
        }
    }
}

pub enum HandType {
    Keyboard,
    Vive,
}

pub const SCREEN_TO_WORLD: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 1.6, -1.0, 1.0f32],
];