use patch::Patch;
use mist::Mist;
use vision::Vision;
use std::boxed::Box;
use std::collections::HashMap;
use common::Wish;
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
