use std::sync::mpsc::{Sender};
use viewer::{Viewer, IdSource, Patch, PatchPosition};
use color;

pub struct Howling {
    is_silenced: bool,
    on_silence: Box<Fn()>,
}

impl Howling {
    pub fn new(on_silence: Box<Fn()>) -> Self {
        Howling { is_silenced: false, on_silence: on_silence }
    }
    pub fn is_silenced(&self) -> bool {
        self.is_silenced
    }
    pub fn silence(&mut self) {
        if self.is_silenced {} else {
            self.is_silenced = true;
            (&self.on_silence)();
        }
    }
}
