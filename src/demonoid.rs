use std::cell::RefCell;
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
    pub model: S::Mdl,
    pub star: Rc<S>,
    pub wish_adapter: RefCell<Option<Rc<Fn(Wish) -> Option<S::Msg>>>>,
}

impl<S: Star> Demonoid<S>
{
    pub fn new(id: u64, model: S::Mdl, star: &S) -> Self {
        Demonoid {
            id: id,
            model: model,
            star: Rc::new((*star).clone()),
            wish_adapter: RefCell::new(None)
        }
    }

    fn get_wish_adapter_option(&self) -> Option<Rc<Fn(Wish) -> Option<S::Msg>>> {
        (*(self.wish_adapter.borrow())).clone()
    }
    fn set_vision_adapter_option(&self, option: Option<Rc<Fn(Wish) -> Option<S::Msg>>>) {
        *self.wish_adapter.borrow_mut() = option;
    }
    fn get_vision_and_save_wish_adapter(&self) -> Vision<S::Msg> {
        let vision: Vision<S::Msg> = self.star.as_ref().view(&self.model);
        self.set_vision_adapter_option(Option::Some(vision.adapter.clone()));
        vision
    }
    fn get_message_from_wish(&self, wish: Wish) -> Option<S::Msg> {
        let vision = self.get_vision_and_save_wish_adapter();
        match wish {
            Wish::Tick => {
                let beats = vision.find_beats(&Instant::now());
                if beats.len() > 0 {
                    let wish_adapter = self.get_wish_adapter_option().unwrap();
                    (*wish_adapter)(wish)
                } else {
                    None
                }
            },
            _ => {
                let wish_adapter = self.get_wish_adapter_option().unwrap();
                (*wish_adapter)(wish)
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
        let vision = self.get_vision_and_save_wish_adapter();
        Box::new(vision)
    }

    fn poke(&mut self, vision_message: Wish) -> DemonResult {
        if let Some(message) = self.get_message_from_wish(vision_message) {
            let mut well = Well::new(|_| None) as Well<S::Out, ()>;
            let model_option = self.star.as_ref().update(&self.model, message, &mut well);
            if let Some(model) = model_option {
                self.model = model;
                self.set_vision_adapter_option(Option::None);
            }
            // TODO Deal with outcomes and wishes
        }
        DemonResult::Keep
    }
}
