use std::cell::RefCell;
use demon::*;
use std::rc::Rc;
use common::Report;
use common::Wish;
use vision::Vision;
use std::time::Instant;

#[derive(Clone)]
pub struct Demonoid<Mod: Clone, Msg: Clone, Out: Clone> {
    pub id: u64,
    pub model: Mod,
    pub update: Rc<Fn(Msg, &Mod) -> Report<Mod, Out>>,
    pub view: Rc<Fn(&Mod) -> Vision<Msg>>,
    pub wish_adapter: RefCell<Option<Rc<Fn(Wish) -> Msg>>>,
}

impl<Mod, Msg, Out> Demonoid<Mod, Msg, Out> where Mod: Clone, Msg: Clone, Out: Clone {
    fn get_wish_adapter_option(&self) -> Option<Rc<Fn(Wish) -> Msg>> {
        (*(self.wish_adapter.borrow())).clone()
    }
    fn set_vision_adapter_option(&self, option: Option<Rc<Fn(Wish) -> Msg>>) {
        *self.wish_adapter.borrow_mut() = option;
    }
    fn get_vision_and_save_wish_adapter(&self) -> Vision<Msg> {
        let vision: Vision<Msg> = (*(self.view))(&self.model);
        self.set_vision_adapter_option(Option::Some(vision.wish_adapter.clone()));
        vision
    }
    fn get_message_from_wish(&self, wish: Wish) -> Option<Msg> {
        let vision = self.get_vision_and_save_wish_adapter();
        match wish {
            Wish::Tick => {
                let beats = vision.find_beats(&Instant::now());
                if beats.len() > 0 {
                    let wish_adapter = self.get_wish_adapter_option().unwrap();
                    let message = (*wish_adapter)(wish);
                    Some(message)
                } else {
                    None
                }
            },
            _ => {
                let wish_adapter = self.get_wish_adapter_option().unwrap();
                let message = (*wish_adapter)(wish);
                Some(message)
            }
        }
    }
}

impl<Mod, Msg, Out> Demon for Demonoid<Mod, Msg, Out> where Mod: 'static + Clone,
                                                            Msg: 'static + Clone,
                                                            Out: 'static + Clone {
    fn clone_and_box(&self) -> Box<Demon> {
        let demonoid: Demonoid<Mod, Msg, Out> = (*self).clone();
        Box::new(demonoid)
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn see(&self) -> Box<DemonVision> {
        let vision = self.get_vision_and_save_wish_adapter();
        Box::new(vision)
    }

    fn poke(&mut self, vision_message: Wish) -> DemonResult {
        match self.get_message_from_wish(vision_message) {
            Some(message) => {
                let report: Report<Mod, Out> = (*(self.update))(message, &self.model);
                match report {
                    Report::Unchanged => DemonResult::Keep,
                    Report::Model(model) => {
                        self.model = model;
                        self.set_vision_adapter_option(Option::None);
                        DemonResult::Keep
                    },
                    Report::Outcome(_) => {
                        // TODO: Should do something with the outcome like pass it on to whoever is expecting it.
                        DemonResult::Remove
                    },
                }
            },
            None => {
                DemonResult::Keep
            }
        }
    }
}
