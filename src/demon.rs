use patch::Patch;
use mist::Mist;
use vision::Vision;
use std::boxed::Box;
use std::collections::HashMap;
use common::Wish;

pub trait Demon {
    fn clone_and_box(&self) -> Box<Demon>;
    fn id(&self) -> u64;
    fn see(&self) -> Box<DemonVision>;
    fn poke(&mut self, vision_outcome: Wish) -> DemonResult;
}

impl Clone for Box<Demon> {
    fn clone(&self) -> Self {
        self.clone_and_box()
    }
}

pub trait DemonVision {
    fn patches(&self) -> &HashMap<u64, Patch>;
    fn mists(&self) -> &HashMap<u64, Mist>;
}

impl<Msg> DemonVision for Vision<Msg> {
    fn patches(&self) -> &HashMap<u64, Patch> {
        &self.patches
    }

    fn mists(&self) -> &HashMap<u64, Mist> {
        &self.mists
    }
}

pub enum DemonResult {
    Keep,
    Remove,
}
