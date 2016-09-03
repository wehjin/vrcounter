mod core;

use std::sync::mpsc::{Sender};
use viewer::{Viewer, IdSource, Patch, PatchPosition};
use color;
use howl::core::{Howling, Message, Howl};

pub fn create_color<T, E>(color: [f32; 4], ) -> Howl<T, E> {
    let (left, right, bottom, top, far, near) = (-0.70, -0.50, -0.10, 0.10, 0.10, 0.10);
    Howl::create(Box::new(move |viewer: Viewer, sender: Sender<Message<T, E>>, id_source: &mut IdSource| -> Howling {
        let position = PatchPosition { left: left, right: right, bottom: bottom, top: top, near: near };
        let id = id_source.next_id();
        let patch = Patch::of_color(&position, &color, id);
        viewer.add_patch(patch);
        sender.send(Message::Position {
            left: left, right: right, bottom: bottom, top: top, far: far, near: near
        }).unwrap();
        Howling::new(Box::new(move || { viewer.remove_patch(id); }))
    }))
}

#[cfg(test)]
mod tests {}