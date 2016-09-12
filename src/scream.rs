extern crate cage;

use patch::{Patch};
use common::Wish;
use std::option::Option;
use vision::Vision;
use cage::Cage;
use star::Star;
use patch::Sigil;

#[derive(Copy, Clone, Debug, Default)]
pub struct Model {
    pub cage_option: Option<Cage>
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Ignore,
    FitToCage(Cage),
}

#[derive(Clone)]
pub struct Scream {
    id: u64,
    color: [f32; 4]
}

impl Star for Scream {
    type Mdl = Model;
    type Msg = Message;
    type Out = ();
    fn init(&self) -> (Self::Mdl, Vec<Wish>) {
        (Model { cage_option: None }, Vec::new())
    }
    fn update(&self, message: Self::Msg, _: &Self::Mdl) -> (Option<Self::Mdl>, Vec<Wish>, Vec<Self::Out>) {
        match message {
            Message::FitToCage(cage) => {
                let next = Model { cage_option: Some(cage) };
                (Some(next), vec![], vec![])
            }
            _ => (None, vec![], vec![])
        }
    }
    fn view(&self, model: &Self::Mdl) -> Vision<Self::Msg> {
        let mut vision = Vision::new(|wish| match wish {
            Wish::FitToCage(cage) => Message::FitToCage(cage),
            _ => Message::Ignore,
        });
        if let Some(cage) = model.cage_option {
            let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, self.id);
            vision.add_patch(patch);
        }
        vision
    }
}

pub fn from_color(id: u64, color: [f32; 4]) -> Scream {
    Scream { id: id, color: color }
}
