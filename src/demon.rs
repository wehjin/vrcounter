use patch::Patch;
use mist::Mist;
use vision::Vision;
use std::boxed::Box;
use std::collections::HashMap;
use common::Wish;
use star::Star;
use std::rc::Rc;

pub trait Sun {
    fn summon(&mut self) -> Vec<Wish>;
    fn see(&mut self) -> Rc<Sight>;
    fn signal(&mut self, Wish) -> (Vec<Wish>, bool);
}

pub trait Sight {
    fn patches(&self) -> &HashMap<u64, Patch>;
    fn mists(&self) -> &HashMap<u64, Mist>;
}

pub trait Flare {
    fn wishes(&self) -> &Vec<Wish>;
}

pub struct StarSun<S: Star> {
    star: S,
    model: Option<S::Mdl>,
    vision_rc_opt: Option<Rc<Vision<S::Msg>>>,
}

impl<S: Star> StarSun<S> {
    fn load_vision(&mut self) -> Rc<Vision<S::Msg>> {
        if let Some(ref vision_rc) = self.vision_rc_opt {
            return vision_rc.clone()
        }
        let vision = match self.model {
            None => Vision::default(),
            Some(ref model) => self.star.view(model),
        };
        let vision_rc = Rc::new(vision);
        self.vision_rc_opt = Some(vision_rc.clone());
        vision_rc
    }
}

impl<S: Star> Sun for StarSun<S> where S::Msg: 'static {
    fn summon(&mut self) -> Vec<Wish> {
        let (model, wishes) = self.star.init();
        self.model = Some(model);
        self.vision_rc_opt = None;
        wishes
    }
    fn see(&mut self) -> Rc<Sight> {
        self.load_vision()
    }

    fn signal(&mut self, wish: Wish) -> (Vec<Wish>, bool) {
        let vision_rc = self.load_vision();
        match vision_rc.as_ref().message_from_wish(wish) {
            None => (vec![], false),
            Some(message) => {
                let (model_op, wishes, outs) = self.star.update(message, self.model.as_ref().unwrap());
                if let Some(model) = model_op {
                    self.model = Some(model);
                }
                    // Inspect out or wishes to determine true/false
                    (wishes, false)
            }
        }
    }
}

impl<T> Sight for Vision<T> {
    fn patches(&self) -> &HashMap<u64, Patch> {
        &self.patches
    }
    fn mists(&self) -> &HashMap<u64, Mist> {
        &self.mists
    }
}

pub trait Demon {
    fn id(&self) -> u64;
    fn see(&self) -> Box<Sight>;
    fn poke(&mut self, wish: Wish) -> DemonResult;
    fn clone_and_box(&self) -> Box<Demon>;
}

impl Clone for Box<Demon> {
    fn clone(&self) -> Self {
        self.clone_and_box()
    }
}

pub enum DemonResult {
    Keep,
    Remove,
}
