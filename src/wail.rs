extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use patch::{Sigil, Patch};

#[derive(Copy, Clone, Debug)]
pub enum WailIn {
    Offset(Offset),
}

#[derive(Copy, Clone, Debug)]
pub enum WailOut {
    Frame(Frame),
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

impl LeafWailModel {
    pub fn view(&self) -> Vision<WailIn> {
        self.wail.view(self)
    }
}

impl LeafWail {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        LeafWail { color: color, frame: frame }
    }
    pub fn summon(self) -> Box<LeafWailModel> {
        let patch_id = rand::random::<u64>();
        let offset = Offset::default();
        Box::new(LeafWailModel { offset: offset, patch_id: patch_id, wail: self })
    }
    pub fn view(&self, model: &LeafWailModel) -> Vision<WailIn> {
        let cage = Cage::from((self.frame, model.offset));
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, model.patch_id);
        let mut vision = Vision::new();
        vision.add_patch(patch);
        vision
    }
    pub fn update(&self, model: &LeafWailModel, message: &WailIn) -> LeafWailModel {
        let mut new_model = (*model).clone();
        match message {
            &WailIn::Offset(offset) => {
                new_model.offset = offset;
            }
        }
        new_model
    }
}
