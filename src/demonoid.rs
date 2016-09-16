use demon::*;
use std::rc::Rc;
use common::Wish;
use vision::Vision;
use report::Well;
use std::time::Instant;
use star::Star;
use std::collections::VecDeque;
use mist::Mist;

#[derive(Clone)]
pub struct Demonoid<S: Star> {
    pub id: u64,
    model: S::Mdl,
    star: Rc<S>,
}

impl<S: Star> Demonoid<S>
{
    pub fn new(id: u64, model: S::Mdl, star: &S) -> Self {
        Demonoid {
            id: id,
            model: model,
            star: Rc::new((*star).clone()),
        }
    }
    fn get_vision(&self) -> Vision<S::Msg> {
        self.star.as_ref().view(&self.model)
    }
    fn get_messages(&self, wish: Wish) -> Vec<S::Msg> {
        let mut messages = Vec::new();
        let vision = self.get_vision();
        match wish.clone() {
            Wish::Tick => {
                let beats = vision.find_beats(&Instant::now());
                for beat in beats {
                    if let Some(message) = vision.get_message_option(beat.id(), wish.clone()) {
                        messages.push(message);
                    }
                }
            },
            Wish::SenseHand(hand) => {
                for (_, mist) in &vision.mists {
                    if let Some(message) = vision.get_message_option(mist.id(), wish.clone()) {
                        messages.push(message);
                    }
                }
            }
            // TODO : Handle SendHand and maybe FitToCage
            _ => ()
        }
        messages
    }
}

impl<S: Star> Demon for Demonoid<S> where S: 'static {
    fn clone_and_box(&self) -> Box<Demon> {
        let demonoid = (*self).clone() as Self;
        Box::new(demonoid)
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn see(&self) -> Box<Sight> {
        let vision = self.get_vision();
        Box::new(vision)
    }

    fn poke(&mut self, wish: Wish) -> DemonResult {
        let mut messages = VecDeque::from(self.get_messages(wish));
        while let Some(message) = messages.pop_front() {
            let mut well = Well::new(|_| None) as Well<S::Out, ()>;
            // TODO Add real well adapter?
            let model_option = self.star.as_ref().update(&self.model, message, &mut well);
            if let Some(model) = model_option {
                self.model = model;
            };
            // TODO Deal with wishes in the well
        }
        DemonResult::Keep
    }
}
