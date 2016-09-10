extern crate cage;

use roar::*;
use summoner::{Report};
use vision::Vision;
use cage::Cage;
use patch::{Patch, Sigil};
use mist::Mist;

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Ignore,
    Silence,
}

impl Default for Message {
    fn default() -> Self {
        Message::Ignore
    }
}

pub fn misty(id: u64, cage: Cage) -> Roar<bool, Message, ()> {
    Roar::create(|| false,
                 |message, is_silenced| if *is_silenced {
                     Report::Unchanged
                 } else {
                     match message {
                         Message::Ignore => Report::Unchanged,
                         Message::Silence => Report::Model(true),
                     }
                 },
                 move |is_silenced| if *is_silenced {
                     Default::default()
                 } else {
                     let mut vision = Vision::create(|vision_outcome| match vision_outcome {
                         _ => Message::Ignore,
                     });
                     vision.add_mist(Mist::new(id, cage));
                     vision
                 })
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
