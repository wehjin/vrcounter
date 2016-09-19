extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use patch::{Sigil, Patch};
use super::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LeafWailModel {
    frame: Frame,
    offset: Offset,
    patch_id: u64,
}

#[derive(Clone, Debug)]
pub struct LeafWail {
    color: [f32; 4],
    frame: Frame,
}

impl Wail for LeafWail {
    type Mdl = LeafWailModel;

    fn update(&self, model: &mut LeafWailModel, message: &WailIn) {
        match message {
            &WailIn::Offset(offset) => {
                model.offset = offset;
            }
        }
    }
    fn view(&self, model: &LeafWailModel) -> Vision<WailIn> {
        let cage = Cage::from((model.frame, model.offset));
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, model.patch_id);
        let mut vision = Vision::new();
        vision.add_patch(patch);
        vision
    }
    fn init(&self) -> LeafWailModel {
        let patch_id = rand::random::<u64>();
        let offset = Offset::default();
        let frame = self.frame.clone();
        LeafWailModel {
            frame: frame,
            offset: offset,
            patch_id: patch_id,
        }
    }
    fn to_subwail(&self) -> Rc<Subwail> {
        Rc::new(LeafSubwail { wail: self.clone(), wail_model: None }) as Rc<Subwail>
    }
}

impl LeafWail {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        LeafWail { color: color, frame: frame }
    }
}

#[derive(Debug)]
pub struct LeafSubwail {
    wail: LeafWail,
    wail_model: Option<LeafWailModel>,
}

// TODO: Make a macro for Subwail.
impl Subwail for LeafSubwail {
    fn report_frame(&self) -> Frame {
        if let Some(ref wail_model) = self.wail_model {
            wail_model.frame.clone()
        } else {
            panic!("Must summon");
        }
    }
    fn update(&mut self, message: &WailIn) {
        if let Some(ref mut wail_model) = self.wail_model {
            self.wail.update(wail_model, message);
        } else {
            panic!("Must summon");
        }
    }
    fn view(&self) -> Vision<WailIn> {
        if let Some(ref wail_model) = self.wail_model {
            self.wail.view(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn summon(&self) -> Wailing {
        if self.wail_model.is_some() {
            panic!("Already summoned");
        } else {
            Wailing {
                subwail: Box::new(LeafSubwail {
                    wail: self.wail.clone(),
                    wail_model: Some(self.wail.init())
                }) as Box<Subwail>
            }
        }
    }
}

