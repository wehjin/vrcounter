extern crate cage;

use roar::*;
use std::rc::Rc;
use summoner::{Report};
use vision::{Vision, VisionMessage};
use cage::Cage;
use patch::{Patch, Sigil};

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Ignore,
    Silence,
}

pub fn create(id: u64, color: [f32; 4], cage: Cage, sigil: Sigil) -> Roar<Cage, Message, ()> {
    Roar {
        init: Rc::new(move || -> Cage {
            cage
        }),
        update: Rc::new(move |message, _: &Cage| -> Report<Cage, ()> {
            match message {
                Message::Silence => Report::Outcome(()),
                Message::Ignore => Report::Unchanged,
            }
        }),
        view: Rc::new(move |cage: &Cage| -> Vision<Message> {
            let mut vision = Vision::create(move |_: VisionMessage| -> Message {
                Message::Ignore
            });
            let patch = Patch::from_cage(cage, color, sigil, id);
            vision.add_patch(patch);
            vision
        }),
    }
}
