extern crate cage;

use patch::{Patch};
use common::Wish;
use star::SeedStar;
use std::option::Option;
use vision::Vision;
use report::Report;
use cage::Cage;

#[derive(Copy, Clone, Debug, Default)]
pub struct Model {
    pub cage_option: Option<Cage>
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Ignore,
    FitToCage(Cage),
}

pub fn from_color(id: u64, color: [f32; 4]) -> SeedStar<Model, Message, ()> {
    use patch::Sigil;
    SeedStar::create(|| Model { cage_option: None },
                     |message, _| match message {
                     Message::FitToCage(cage) => {
                         let next = Model { cage_option: Some(cage) };
                         Report::Model::<Model, ()>(next)
                     }
                     _ => Report::Unchanged,
                 },
                     move |model: &Model| {
                     let mut vision = Vision::create(|wish| match wish {
                         Wish::FitToCage(cage) => Message::FitToCage(cage),
                         _ => Message::Ignore,
                     });
                     if let Some(cage) = model.cage_option {
                         let patch = Patch::from_cage(&cage, color, Sigil::Fill, id);
                         vision.add_patch(patch);
                     }
                     vision
                 })
}
