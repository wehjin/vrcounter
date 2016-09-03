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

pub enum Message<T, E> {
    Position {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        far: f32,
        near: f32,
    },
    Ok(T),
    Err(E),
}

pub struct Howl<T, E> {
    on_present: Box<Fn(Viewer, Sender<Message<T, E>>, &mut IdSource) -> Howling>,
}

impl<T, E> Howl<T, E> {
    pub fn create(on_present: Box<Fn(Viewer, Sender<Message<T, E>>, &mut IdSource) -> Howling>) -> Self {
        Howl { on_present: on_present }
    }
    pub fn present(&self, viewer: Viewer, sender: Sender<Message<T, E>>, id_source: &mut IdSource) -> Howling {
        let on_present = &(self.on_present);
        on_present(viewer, sender, id_source)
    }
}
