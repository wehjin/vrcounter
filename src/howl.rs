extern crate cage;

use star::SeedStar;
use vision::Vision;
use cage::Cage;
use patch::{Patch, Sigil};
use mist::Mist;
use report::Report;

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

pub fn misty(id: u64, cage: Cage) -> SeedStar<bool, Message, ()> {
    use summoner::Summoner;
    use std::rc::Rc;
    use common::IdSource;
    use common::Wish;
    use color::WHITE;

    fn init() -> (bool, Vec<Wish>) {
        fn summon(id_source: &mut IdSource, summoner: &mut Summoner) {
            let cage = Cage::from((-0.7, -0.5, 0.25, 0.45, 0.25, 0.25));
            let sub_star = create(id_source.id(), WHITE, cage, Sigil::Letter('S'));
            summoner.summon(id_source, &sub_star, |_| false);
        }

        (false, vec![Wish::SummonStar(Rc::new(summon))])
    }

    fn update(message: Message, is_silenced: &bool) -> Report<bool, ()> {
        if *is_silenced {
            Report::Unchanged
        } else {
            match message {
                Message::Ignore => Report::Unchanged,
                Message::Silence => Report::Model(true),
            }
        }
    }
    SeedStar::create(init, update,
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

pub fn create(id: u64, color: [f32; 4], cage: Cage, sigil: Sigil) -> SeedStar<Cage, Message, ()> {
    SeedStar::create(
        move || (cage, vec![]),
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
