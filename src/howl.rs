extern crate cage;
extern crate rand;

use star::Star;
use vision::Vision;
use cage::Cage;
use patch::{Patch, Sigil};
use mist::Mist;
use color::WHITE;
use report::Well;

#[derive(Clone)]
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
        sub_star: create(rand::random::<u64>(),
                         WHITE,
                         Cage::from((-0.7, -0.5, 0.25, 0.45, 0.25, 0.25)),
                         Sigil::Letter('S')),
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Silence,
}

#[derive(Clone)]
pub struct Misty {
    pub is_silenced: bool,
    pub sub_model: Cage,
}

impl Star for MistyStar {
    type Mdl = Misty;
    type Msg = Message;
    type Out = ();

    fn init(&self) -> Misty {
        let misty = Misty {
            is_silenced: false,
            sub_model: self.sub_star.init(),
        };
        misty
    }

    fn view(&self, model: &Misty) -> Vision<Message> {
        if model.is_silenced {
            Default::default()
        } else {
            let sub_vision = self.sub_star.view(&model.sub_model);
            let mut vision = Vision::new(|_| None);
            vision.add_mist(Mist::new(self.id, self.cage));
            // TODO Adapt wishes intended for sub_vision into Message::ForSubStar.
            vision.add_vision(sub_vision);
            vision
        }
    }

    fn update<T>(&self, model: &Misty, message: Message, well: &mut Well<(), T>) -> Option<Misty> {
        if model.is_silenced {
            return None;
        }
        match message {
            Message::Silence => {
                let mut clone = model.clone();
                clone.is_silenced = true;
                Some(clone)
            },
        }
    }
}

#[derive(Clone)]
pub struct Howl {
    id: u64,
    color: [f32; 4],
    cage: Cage,
    sigil: Sigil
}

impl Star for Howl {
    type Mdl = Cage;
    type Msg = Message;
    type Out = ();

    fn init(&self) -> Cage {
        self.cage
    }

    fn update<T>(&self, _: &Cage, _: Message, _: &mut Well<(), T>) -> Option<Cage> {
        None
    }

    fn view(&self, model: &Cage) -> Vision<Message> {
        let mut vision = Vision::new(move |_| None);
        let patch = Patch::from_cage(&model, self.color, self.sigil, self.id);
        vision.add_patch(patch);
        vision
    }
}

pub fn create(id: u64, color: [f32; 4], cage: Cage, sigil: Sigil) -> Howl {
    Howl { id: id, color: color, cage: cage, sigil: sigil }
}
