mod core;

use std::sync::mpsc::{Sender};
use viewer::{ActiveViewer};
use common::IdSource;
use patch::{PatchPosition, Patch};
pub use patch::Sigil;
pub use howl::core::{Howling, Message, Howl};

pub fn start<T, E>(color: [f32; 4], sigil: Sigil, (left, right, bottom, top, far, near): (f32, f32, f32, f32, f32, f32)) -> Howl<T, E> {
    Howl::create(Box::new(move |viewer: ActiveViewer, sender: Sender<Message<T, E>>, id_source: &mut IdSource| -> Howling {
        let id = id_source.next_id();
        let patch = Patch::new(id, left, right, bottom, top, far, color, sigil);
        viewer.add_patch(patch);
        sender.send(Message::Position {
            left: left, right: right, bottom: bottom, top: top, far: far, near: near
        }).unwrap();
        Howling::new(Box::new(move || { viewer.remove_patch(id); }))
    }))
}

#[cfg(test)]
mod tests {}