use demon::*;
use std::rc::Rc;
use common::Wish;
use vision::Vision;
use report::Well;
use std::time::Instant;
use star::Star;

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
    fn get_message_from_wish(&self, wish: Wish) -> Option<S::Msg> {
        let vision = self.get_vision();
        match wish {
            Wish::Tick => {
                let beats = vision.find_beats(&Instant::now());
                if beats.len() > 0 {
                    vision.get_message_option(wish)
                } else {
                    None
                }
            },
            _ => {
                vision.get_message_option(wish)
            }
        }
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

    fn poke(&mut self, vision_message: Wish) -> DemonResult {
        if let Some(message) = self.get_message_from_wish(vision_message) {
            let mut well = Well::new(|_| None) as Well<S::Out, ()>;
            let model_option = self.star.as_ref().update(&self.model, message, &mut well);
            if let Some(model) = model_option {
                self.model = model;
            }
            // TODO Deal with outcomes and wishes
        }
        DemonResult::Keep
    }
}
