extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use patch::{Sigil, Patch};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub enum WailIn {
    Offset(Offset),
}

#[derive(Copy, Clone, Debug)]
pub enum WailOut {
    Frame(Frame),
}

pub trait Wail: Clone {
    type Mdl: Wailing;
    fn update(&self, model: &Self::Mdl, message: &WailIn) -> Self::Mdl;
    fn view(&self, model: &Self::Mdl) -> Vision<WailIn>;
    fn summon(self) -> Self::Mdl;
}

pub trait Wailing: Debug {
    fn view(&self) -> Vision<WailIn>;
}


#[derive(Clone, Debug)]
pub struct LeafWail {
    color: [f32; 4],
    frame: Frame,
}

#[derive(Clone, Debug)]
pub struct LeafWailModel {
    offset: Offset,
    patch_id: u64,
    wail: LeafWail,
}

impl Wailing for LeafWailModel {
    fn view(&self) -> Vision<WailIn> {
        self.wail.view(self)
    }
}

impl LeafWail {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        LeafWail { color: color, frame: frame }
    }
}

impl Wail for LeafWail {
    type Mdl = LeafWailModel;

    fn summon(self) -> LeafWailModel {
        let patch_id = rand::random::<u64>();
        let offset = Offset::default();
        LeafWailModel { offset: offset, patch_id: patch_id, wail: self }
    }
    fn view(&self, model: &LeafWailModel) -> Vision<WailIn> {
        let cage = Cage::from((self.frame, model.offset));
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, model.patch_id);
        let mut vision = Vision::new();
        vision.add_patch(patch);
        vision
    }
    fn update(&self, model: &LeafWailModel, message: &WailIn) -> LeafWailModel {
        let mut new_model = (*model).clone();
        match message {
            &WailIn::Offset(offset) => {
                new_model.offset = offset;
            }
        }
        new_model
    }
}
