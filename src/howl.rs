extern crate cage;

use star::Star;
use vision::Vision;
use cage::Cage;
use patch::{Patch, Sigil};
use mist::Mist;
use summoner::Summoner;
use std::rc::Rc;
use common::IdSource;
use common::Wish;
use color::WHITE;

#[derive(Clone)]
pub struct MistyStar {
    id: u64,
    cage: Cage
}

pub fn misty(id: u64, cage: Cage) -> MistyStar {
    MistyStar { id: id, cage: cage }
}

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Silence,
}

#[derive(Clone)]
pub struct Misty {
    pub is_silenced: bool
}

fn summon(id_source: &mut IdSource, summoner: &mut Summoner) {
    let id = id_source.id();
    let cage = Cage::from((-0.7, -0.5, 0.25, 0.45, 0.25, 0.25));
    let sub_star = create(id, WHITE, cage, Sigil::Letter('S'));
    summoner.summon(id_source, &sub_star, |_| false);
}

impl Star for MistyStar {
    type Mdl = Misty;
    type Msg = Message;
    type Out = ();

    fn init(&self) -> (Misty, Vec<Wish>) {
        let mut wishes = Vec::new();
        wishes.push(Wish::SummonStar(Rc::new(summon)));
        (Misty { is_silenced: false }, wishes)
    }

    fn update(&self, message: Message, model: &Misty) -> (Option<Misty>, Vec<Wish>, Vec<()>) {
        if model.is_silenced {
            (None, vec![], vec![])
        } else {
            match message {
                Message::Silence => {
                    let mut clone = model.clone();
                    clone.is_silenced = true;
                    (Some(clone), vec![], vec![])
                },
            }
        }
    }

    fn view(&self, model: &Misty) -> Vision<Message> {
        if model.is_silenced {
            Default::default()
        } else {
            let mut vision = Vision::new(|_| None);
            vision.add_mist(Mist::new(self.id, self.cage));
            vision
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

    fn init(&self) -> (Self::Mdl, Vec<Wish>) {
        (self.cage, vec![])
    }

    fn update(&self, _: Self::Msg, _: &Self::Mdl) -> (Option<Self::Mdl>, Vec<Wish>, Vec<Self::Out>) {
        (None, vec![], vec![])
    }

    fn view(&self, model: &Self::Mdl) -> Vision<Self::Msg> {
        let mut vision = Vision::new(move |_| None);
        let patch = Patch::from_cage(&model, self.color, self.sigil, self.id);
        vision.add_patch(patch);
        vision
    }
}

pub fn create(id: u64, color: [f32; 4], cage: Cage, sigil: Sigil) -> Howl {
    Howl { id: id, color: color, cage: cage, sigil: sigil }
}
