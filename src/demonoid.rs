use demon::*;
use std::rc::Rc;
use common::Wish;
use vision::Vision;
use std::time::Instant;
use star::Star;
use std::collections::VecDeque;

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
            Wish::SenseHand(_) => {
                for (_, mist) in &vision.mists {
                    if let Some(message) = vision.get_message_option(mist.id(), wish.clone()) {
                        messages.push(message);
                    }
                }
            }
            // TODO : Handle FitToCage
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
        let messages = self.get_messages(wish);
        if messages.len() > 0 {
            let mut queue = VecDeque::from(messages);
            while let Some(ref message) = queue.pop_front() {
                let new_model = self.star.as_ref().update(&self.model, message);
                self.model = new_model;
            }
        }
        DemonResult::Keep
    }
}
