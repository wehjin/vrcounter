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

#[derive(Copy, Clone, Debug)]
pub enum Message {
    Silence,
}

#[derive(Clone)]
pub struct MistyStar {
    id: u64,
    cage: Cage
}

fn summon(id_source: &mut IdSource, summoner: &mut Summoner) {
    let id = id_source.id();
    let cage = Cage::from((-0.7, -0.5, 0.25, 0.45, 0.25, 0.25));
    let sub_star = create(id, WHITE, cage, Sigil::Letter('S'));
    summoner.summon(id_source, &sub_star, |_| false);
}

impl Star for MistyStar {
    type Mdl = bool;
    type Msg = Message;
    type Out = ();

    fn init(&self) -> (Self::Mdl, Vec<Wish>) {
        let mut wishes = Vec::new();
        wishes.push(Wish::SummonStar(Rc::new(summon)));
        (false, wishes)
    }

    fn update(&self, message: Self::Msg, is_silenced: &Self::Mdl) -> (Option<Self::Mdl>, Vec<Wish>, Vec<Self::Out>) {
        if *is_silenced {
            return (None, vec![], vec![])
        }
        match message {
            Message::Silence => (Some(true), vec![], vec![]),
        }
    }

    fn view(&self, is_silenced: &Self::Mdl) -> Vision<Self::Msg> {
        if *is_silenced {
            Default::default()
        } else {
            let mut vision = Vision::new(|_| None);
            vision.add_mist(Mist::new(self.id, self.cage));
            vision
        }
    }
}

pub fn misty(id: u64, cage: Cage) -> MistyStar {
    MistyStar { id: id, cage: cage }
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
