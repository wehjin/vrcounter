use patch::Patch;
use mist::Mist;
use vision::Vision;
use report::Well;
use std::boxed::Box;
use std::collections::HashMap;
use common::Wish;
use star::Star;
use std::rc::Rc;

pub trait Sun {
    fn summon(&mut self);
    fn see(&mut self) -> Rc<Sight>;
    fn signal(&mut self, Wish) -> bool;
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
    fn summon(&mut self) {
        let model = self.star.init();
        self.model = Some(model);
        self.vision_rc_opt = None;
    }
    fn see(&mut self) -> Rc<Sight> {
        self.load_vision()
    }
    fn signal(&mut self, wish: Wish) -> bool {
        let vision_rc = self.load_vision();
        match vision_rc.as_ref().get_message_option(wish) {
            None => false,
            Some(message) => {
                let mut well = Well::new(|x| None) as Well<S::Out, bool>;
                let model_op = self.star.update(self.model.as_ref().unwrap(), message, &mut well);
                if let Some(model) = model_op {
                    self.model = Some(model);
                };
                // TODO deal with messages in well.
                // TODO deal with wishes in well.
                false
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
