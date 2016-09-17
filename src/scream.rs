extern crate cage;

use patch::{Patch};
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
    FitToCage(Cage),
}

#[derive(Clone, Debug)]
pub struct Scream {
    id: u64,
    color: [f32; 4]
}

impl Star for Scream {
    type Mdl = Model;
    type Msg = Message;
    type Out = ();
    fn init(&self) -> Self::Mdl {
        Model { cage_option: None }
    }
    fn update(&self, _: &Model, message: &Message) -> Model {
        match message {
            &Message::FitToCage(cage) => Model { cage_option: Some(cage) }
        }
    }
    fn view(&self, model: &Self::Mdl) -> Vision<Self::Msg> {
        let mut vision = Vision::new();
        if let Some(cage) = model.cage_option {
            let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, self.id);
            vision.add_patch(patch);
        }
        vision
    }
}

pub fn new(id: u64, color: [f32; 4]) -> Scream {
    Scream { id: id, color: color }
}
