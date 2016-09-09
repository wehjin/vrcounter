extern crate cage;

use roar::*;
use summoner::{Report};
use vision::{Vision};
use cage::Cage;
use patch::{Patch, Sigil};

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Ignore,
    Silence,
}

pub fn create(id: u64, color: [f32; 4], cage: Cage, sigil: Sigil) -> Roar<Cage, Message, ()> {
    Roar::create(
        move || -> Cage { cage },
        |message, _| match message {
            Message::Silence => Report::Outcome(()),
            Message::Ignore => Report::Unchanged,
        },
        move |cage| {
            let mut vision = Vision::create(move |_| Message::Ignore);
            let patch = Patch::from_cage(cage, color, sigil, id);
            vision.add_patch(patch);
            vision
        }
    )
}
