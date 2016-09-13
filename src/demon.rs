use patch::Patch;
use mist::Mist;
use vision::Vision;
use std::boxed::Box;
use std::collections::HashMap;
use common::Wish;
use star::Star;

pub trait Sun {
    fn summon(&mut self) -> Vec<Wish>;
    fn see(&mut self) -> Box<Sight>;
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
}

impl<S: Star> Sun for StarSun<S> where S::Msg: 'static {
    fn summon(&mut self) -> Vec<Wish> {
        let (model, wishes) = self.star.init();
        self.model = Some(model);
        wishes
    }
    fn see(&mut self) -> Box<Sight> {
        let vision = match self.model {
            None => Vision::default(),
            Some(ref model) => self.star.view(model),
        };
        Box::new(vision)
    }

    fn signal(&mut self, _: Wish) -> (Vec<Wish>, bool) {
        unimplemented!()
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
