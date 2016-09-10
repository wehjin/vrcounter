use std::boxed::Box;
use std::rc::Rc;
use std::collections::HashMap;
use patch::Patch;
use mist::Mist;
use common::IdSource;
use std::cell::RefCell;
use roar::Roar;
use vision;
use vision::Vision;
use std::time::Instant;

pub enum DemonResult {
    Keep,
    Remove,
}

pub trait DemonVision {
    fn patches(&self) -> &HashMap<u64, Patch>;
    fn mists(&self) -> &HashMap<u64, Mist>;
}

pub trait Demon {
    fn clone_and_box(&self) -> Box<Demon>;
    fn id(&self) -> u64;
    fn see(&self) -> Box<DemonVision>;
    fn poke(&mut self, vision_outcome: vision::Outcome) -> DemonResult;
}

#[derive(Clone)]
pub struct Demonoid<Mod: Clone, Msg: Clone, Out: Clone> {
    id: u64,
    model: Mod,
    update: Rc<Fn(Msg, &Mod) -> Report<Mod, Out>>,
    view: Rc<Fn(&Mod) -> Vision<Msg>>,
    vision_message_adapter: RefCell<Option<Rc<Fn(vision::Outcome) -> Msg>>>,
}

impl<Mod, Msg, Out> Demonoid<Mod, Msg, Out> where Mod: Clone, Msg: Clone, Out: Clone {
    fn get_vision_adapter_option(&self) -> Option<Rc<Fn(vision::Outcome) -> Msg>> {
        (*(self.vision_message_adapter.borrow())).clone()
    }
    fn set_vision_adapter_option(&self, option: Option<Rc<Fn(vision::Outcome) -> Msg>>) {
        *self.vision_message_adapter.borrow_mut() = option;
    }
    fn get_vision_and_save_vision_message_adapter(&self) -> Vision<Msg> {
        let vision: Vision<Msg> = (*(self.view))(&self.model);
        self.set_vision_adapter_option(Option::Some(vision.vision_message_adapter.clone()));
        vision
    }
    fn get_message_from_vision_message(&self, vision_message: vision::Outcome) -> Option<Msg> {
        match vision_message {
            vision::Outcome::Tick => {
                let vision = self.get_vision_and_save_vision_message_adapter();
                let beats = vision.find_beats(&Instant::now());
                if beats.len() > 0 {
                    let vision_message_adapter = self.get_vision_adapter_option().unwrap();
                    let message = (*vision_message_adapter)(vision_message);
                    Some(message)
                } else {
                    None
                }
            },
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
        let vision = self.get_vision_and_save_vision_message_adapter();
        Box::new(vision)
    }

    fn poke(&mut self, vision_message: vision::Outcome) -> DemonResult {
        match self.get_message_from_vision_message(vision_message) {
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
                    Report::Error => {
                        println!("Error while poking demon");
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

impl Clone for Box<Demon> {
    fn clone(&self) -> Self {
        self.clone_and_box()
    }
}


#[derive(Clone)]
pub struct Summoner {
    pub demons: HashMap<u64, Box<Demon>>,
}

impl Summoner {
    pub fn new() -> Self {
        Summoner { demons: HashMap::new() }
    }
    pub fn get_demon_boxes(&self) -> Vec<&Box<Demon>> {
        let mut demon_boxes = Vec::new();
        for (_, demon_box) in &self.demons {
            demon_boxes.push(demon_box);
        }
        demon_boxes
    }
    pub fn summon<Msg, SubMod, SubMsg, SubOut, F>(&mut self,
                                                  id_source: &mut IdSource,
                                                  roar: &Roar<SubMod, SubMsg, SubOut>,
                                                  outcome_adapter: F) -> u64
                                                  where SubMod: 'static + Clone,
                                                        SubMsg: 'static + Clone,
                                                        SubOut: 'static + Clone,
                                                        F: Fn(SubOut) -> Msg + 'static {
        let model = ((*roar).init)();
        let id = id_source.id();
        let demon = Demonoid {
            id: id,
            model: model,
            update: roar.update.clone(),
            view: roar.view.clone(),
            vision_message_adapter: RefCell::new(Option::None),
        };
        self.demons.insert(id, Box::new(demon));
        id
    }
    pub fn update(&mut self, vision_message: vision::Outcome) {
        let mut new_demons = HashMap::new();
        for (_, demon_box) in &self.demons {
            let mut new_demon_box = demon_box.clone();
            let demon_result = new_demon_box.poke(vision_message);
            match demon_result {
                DemonResult::Keep => {
                    new_demons.insert(new_demon_box.id(), new_demon_box);
                },
                DemonResult::Remove => (),
            }
        }
        self.demons = new_demons;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demons() {
        let summoner = Summoner::new();
        let demons: Vec<&Box<Demon>> = summoner.get_demon_boxes();
        assert_eq!(0, demons.len());
    }
}

impl<Msg> DemonVision for Vision<Msg> {
    fn patches(&self) -> &HashMap<u64, Patch> {
        &self.patches
    }

    fn mists(&self) -> &HashMap<u64, Mist> {
        &self.mists
    }
}

pub enum Report<Mod, Out> {
    Unchanged,
    Model(Mod),
    Outcome(Out),
    Error,
}
