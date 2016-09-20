extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use patch::{Sigil, Patch};
use super::*;
use std::rc::Rc;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct LeafWailerModel {
    frame: Frame,
    offset: Offset,
    patch_id: u64,
}

#[derive(Clone, Debug)]
pub struct LeafWailer {
    color: [f32; 4],
    frame: Frame
}

impl LeafWailer {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        LeafWailer { color: color, frame: frame }
    }
}

impl Wailer<()> for LeafWailer {
    type Mdl = LeafWailerModel;

    fn report(&self, model: &LeafWailerModel) -> Vec<()> {
        vec![]
    }
    fn update(&self, model: &mut LeafWailerModel, message: &WailerIn) {
        match message {
            &WailerIn::Offset(offset) => {
                model.offset = offset;
            }
        }
    }
    fn view(&self, model: &LeafWailerModel) -> Vision<WailerIn> {
        let cage = Cage::from((model.frame, model.offset));
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, model.patch_id);
        let mut vision = Vision::new();
        vision.add_patch(patch);
        vision
    }
    fn init(&self) -> LeafWailerModel {
        let patch_id = rand::random::<u64>();
        let offset = Offset::default();
        let frame = self.frame.clone();
        LeafWailerModel {
            frame: frame,
            offset: offset,
            patch_id: patch_id,
        }
    }
    fn to_subwail(&self) -> Rc<Subwailer<()>> {
        Rc::new(LeafSubwailer { wail: self.clone(), wail_model: None }) as Rc<Subwailer<()>>
    }
}

#[derive(Debug)]
pub struct LeafSubwailer {
    wail: LeafWailer,
    wail_model: Option<LeafWailerModel>,
}

impl Subwailer<()> for LeafSubwailer {
    fn report(&self) -> Vec<()> {
        if let Some(ref wail_model) = self.wail_model {
            self.wail.report(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn report_frame(&self) -> Frame {
        if let Some(ref wail_model) = self.wail_model {
            wail_model.frame.clone()
        } else {
            panic!("Must summon");
        }
    }
    fn update(&mut self, message: &WailerIn) {
        if let Some(ref mut wail_model) = self.wail_model {
            self.wail.update(wail_model, message);
        } else {
            panic!("Must summon");
        }
    }
    fn view(&self) -> Vision<WailerIn> {
        if let Some(ref wail_model) = self.wail_model {
            self.wail.view(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn summon(&self) -> Wailing<()> {
        if self.wail_model.is_some() {
            panic!("Already summoned");
        } else {
            Wailing {
                subwail: Box::new(LeafSubwailer {
                    wail: self.wail.clone(),
                    wail_model: Some(self.wail.init())
                }) as Box<Subwailer<()>>
            }
        }
    }
}