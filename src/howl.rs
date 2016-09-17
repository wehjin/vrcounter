extern crate cage;
extern crate rand;

use star::Star;
use vision::Vision;
use cage::Cage;
use patch::{Patch, Sigil};
use mist::Mist;
use color::WHITE;

#[derive(Clone, Debug)]
pub struct MistyStar {
    id: u64,
    cage: Cage,
    sub_star: Howl
}

// TODO Make sub_star into a LetterStar.
pub fn misty(id: u64, cage: Cage) -> MistyStar {
    MistyStar {
        id: id,
        cage: cage,
        sub_star: Howl::new(rand::random::<u64>(),
                            WHITE,
                            Cage::from((-0.7, -0.5, 0.25, 0.45, 0.25, 0.25)),
                            Sigil::Letter('S')),
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MistyMessage {
    Silence,
    Forward(Message),
}

#[derive(Clone, Debug)]
pub struct Misty {
    pub is_silenced: bool,
    pub sub_model: Cage,
}

impl Star for MistyStar {
    type Mdl = Misty;
    type Msg = MistyMessage;
    type Out = ();

    fn init(&self) -> Misty {
        let misty = Misty {
            is_silenced: false,
            sub_model: self.sub_star.init(),
        };
        misty
    }

    fn view(&self, model: &Misty) -> Vision<MistyMessage> {
        if model.is_silenced {
            Default::default()
        } else {
            let sub_vision = self.sub_star.view(&model.sub_model);
            let mut vision = Vision::new();
            vision.add_mist(Mist::new(self.id, self.cage), |_| None);
            vision.add_vision(sub_vision, |x| Some(MistyMessage::Forward(x)));
            vision
        }
    }

    fn update(&self, model: &Misty, message: &MistyMessage) -> Misty {
        if model.is_silenced {
            return model.clone();
        }
        match message {
            &MistyMessage::Silence => {
                let mut clone = model.clone();
                clone.is_silenced = true;
                clone
            },
            &MistyMessage::Forward(ref howl_message) => {
                let mut new_model = model.clone();
                new_model.sub_model = self.sub_star.update(&model.sub_model, howl_message);
                new_model
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Howl {
    id: u64,
    color: [f32; 4],
    cage: Cage,
    sigil: Sigil
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Silence,
}

impl Star for Howl {
    type Mdl = Cage;
    type Msg = Message;
    type Out = ();

    fn init(&self) -> Cage {
        self.cage
    }

    fn update(&self, cage: &Cage, _: &Message) -> Cage {
        cage.clone()
    }

    fn view(&self, model: &Cage) -> Vision<Message> {
        let mut vision = Vision::new();
        vision.add_patch(Patch::from_cage(&model, self.color, self.sigil, self.id));
        vision
    }
}

impl Howl {
    pub fn new(id: u64, color: [f32; 4], cage: Cage, sigil: Sigil) -> Howl {
        Howl { id: id, color: color, cage: cage, sigil: sigil }
    }
}

