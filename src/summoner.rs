use std::boxed::Box;
use std::collections::HashMap;
use common::IdSource;
use std::cell::RefCell;
use star::SeedStar;
use demon::Demon;
use demon::DemonResult;
use demonoid::Demonoid;
use common::Wish;
use std::rc::Rc;

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
    pub fn get_demon_box_clone(&self, id: u64) -> Option<Box<Demon>> {
        if let Some(demon_box) = self.demons.get(&id) {
            Some(demon_box.clone())
        } else {
            None
        }
    }

    pub fn summon<Msg, SubMod, SubMsg, SubOut, F>(&mut self,
                                                  id_source: &mut IdSource,
                                                  star: &SeedStar<SubMod, SubMsg, SubOut>,
                                                  outcome_adapter: F) -> u64
                                                  where SubMod: 'static + Clone,
                                                        SubMsg: 'static + Clone,
                                                        SubOut: 'static + Clone,
                                                        F: Fn(SubOut) -> Msg + 'static,
                                                        Self: Sized {
        let (model, wishes) = ((*star).init)();
        let id = id_source.id();
        let demon = Demonoid {
            id: id,
            model: model,
            update: star.update.clone(),
            view: star.view.clone(),
            wish_adapter: RefCell::new(Option::None),
        };
        self.demons.insert(id, Box::new(demon));

        // TODO: Add to queue.
        for wish in wishes {
            if let Wish::SummonStar(ref summon) = wish {
                summon(id_source, self);
            }
        }
        id
    }
    pub fn update_one(&mut self, id: u64, wish: Wish) {
        let demon_box_option = self.get_demon_box_clone(id);
        if let Some(mut demon_box) = demon_box_option {
            match demon_box.poke(wish) {
                DemonResult::Keep => {
                    self.demons.insert(id, demon_box);
                },
                DemonResult::Remove => {
                    self.demons.remove(&id);
                },
            }
        }
    }
    pub fn update(&mut self, wish: Wish) {
        let mut new_demons = HashMap::new();
        for (_, demon_box) in &self.demons {
            let mut new_demon_box = demon_box.clone();
            let demon_result = new_demon_box.poke(wish.clone());
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

