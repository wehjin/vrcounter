use std::boxed::Box;
use std::rc::Rc;
use std::collections::HashMap;
use patch::Patch;
use mist::Mist;
use common::IdSource;
use std::cell::RefCell;
use roar::Roar;
use vision::{Vision, VisionMessage};

pub enum DemonResult {
    Keep,
    Remove,
}

pub trait DemonVision {
    fn patches(&self) -> &HashMap<u64, Patch>;
    fn mists(&self) -> &HashMap<u64, Mist>;
}

pub trait Demon {
    fn id(&self) -> u64;
    fn see(&self) -> Box<DemonVision>;
    fn poke(&mut self, vision_message: VisionMessage) -> DemonResult;
}

pub struct Demonoid<Mod, Msg, Out> {
    id: u64,
    model: Mod,
    update: Rc<Fn(Msg, &Mod) -> Report<Mod, Out>>,
    view: Rc<Fn(&Mod) -> Vision<Msg>>,
    vision_message_adapter: RefCell<Option<Rc<Fn(VisionMessage) -> Msg>>>,
}

impl<Mod, Msg, Out> Demonoid<Mod, Msg, Out> {
    fn get_vision_adapter_option(&self) -> Option<Rc<Fn(VisionMessage) -> Msg>> {
        (*(self.vision_message_adapter.borrow())).clone()
    }
    fn set_vision_adapter_option(&self, option: Option<Rc<Fn(VisionMessage) -> Msg>>) {
        *self.vision_message_adapter.borrow_mut() = option;
    }
    fn get_vision_and_save_vision_message_adapter(&self) -> Vision<Msg> {
        let vision: Vision<Msg> = (*(self.view))(&self.model);
        self.set_vision_adapter_option(Option::Some(vision.vision_message_adapter.clone()));
        vision
    }
    fn get_message_from_vision_message(&self, vision_message: VisionMessage) -> Msg {
        if let Option::None = self.get_vision_adapter_option() {
            self.get_vision_and_save_vision_message_adapter();
        }
        let vision_adapter_ref: Rc<Fn(VisionMessage) -> Msg> = self.get_vision_adapter_option().unwrap();
        (*vision_adapter_ref)(vision_message)
    }
}

impl<Mod, Msg: 'static, Out> Demon for Demonoid<Mod, Msg, Out> {
    fn id(&self) -> u64 {
        self.id
    }

    fn see(&self) -> Box<DemonVision> {
        let vision = self.get_vision_and_save_vision_message_adapter();
        Box::new(vision)
    }

    fn poke(&mut self, vision_message: VisionMessage) -> DemonResult {
        let message = self.get_message_from_vision_message(vision_message);
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
    }
}


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
    pub fn summon<Msg, SubMod: 'static, SubMsg: 'static, SubOut: 'static>(
        &mut self,
        id_source: &mut IdSource,
        roar: &Roar<SubMod, SubMsg, SubOut>,
        outcome_adapter: Box<Fn(SubOut) -> Msg>
    ) -> u64 {
        let model = ((*roar).init)();
        let id = id_source.next_id();
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
