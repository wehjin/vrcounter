use std::rc::Rc;
use glium::{Display, Surface};

pub struct KeyshieldProgram;

impl KeyshieldProgram {
    pub fn new(_display: Rc<Display>) -> Self {
        KeyshieldProgram {}
    }

    pub fn draw<T: Surface>(&self, _surface: &mut T, _view: &[[f32; 4]; 4], _projection: &[[f32; 4]; 4]) {}
}