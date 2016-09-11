use std::boxed::Box;
use std::collections::HashMap;
use common::IdSource;
use std::cell::RefCell;
use roar::Roar;
use vision;
use demon::Demon;
use demon::DemonResult;
use demonoid::Demonoid;


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
    use demon::Demon;

    #[test]
    fn demons() {
        let summoner = Summoner::new();
        let demons: Vec<&Box<Demon>> = summoner.get_demon_boxes();
        assert_eq!(0, demons.len());
    }
}

