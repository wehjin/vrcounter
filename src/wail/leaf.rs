extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use patch::{Sigil, Patch};
use super::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct LeafWailerModel {
    frame: Frame,
    offset: Offset,
    patch_id: u64,
}

#[derive(Clone, Debug)]
pub struct LeafWailer {
    color: [f32; 4],
    frame: Frame,
}

impl Wailer for LeafWailer {
    type Mdl = LeafWailerModel;

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
    fn to_subwail(&self) -> Rc<Subwailer> {
        Rc::new(LeafSubwailer { wail: self.clone(), wail_model: None }) as Rc<Subwailer>
    }
}

impl LeafWailer {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        LeafWailer { color: color, frame: frame }
    }
}

subwail!(LeafSubwailer, LeafWailer, LeafWailerModel);
