extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use patch::{Sigil, Patch};
use super::*;

#[derive(Clone, Debug)]
pub struct LeafWail {
    color: [f32; 4],
    frame: Frame,
}

#[derive(Debug)]
pub struct LeafWailing {
    leaf_wail: LeafWail,
    frame: Frame,
    offset: Offset,
    patch_id: u64,
}

impl Wailing for LeafWailing {
    fn update(&mut self, message: &WailIn) {
        let leaf_wail = self.leaf_wail.clone();
        leaf_wail.update(self, message);
    }
    fn view(&self) -> Vision<WailIn> {
        self.leaf_wail.view(self)
    }
    fn report_frame(&self) -> Frame {
        self.frame.clone()
    }
}

impl LeafWail {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        let wail_box :Option<Box<Wail<Mdl=LeafWailing>>> = None;
        LeafWail { color: color, frame: frame }
    }
}

impl Wail for LeafWail {
    type Mdl = LeafWailing;

    fn update(&self, model: &mut LeafWailing, message: &WailIn) {
        match message {
            &WailIn::Offset(offset) => {
                model.offset = offset;
            }
        }
    }
    fn view(&self, model: &LeafWailing) -> Vision<WailIn> {
        let cage = Cage::from((model.frame, model.offset));
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, model.patch_id);
        let mut vision = Vision::new();
        vision.add_patch(patch);
        vision
    }
    fn summon(self) -> LeafWailing {
        let patch_id = rand::random::<u64>();
        let offset = Offset::default();
        let frame = self.frame.clone();
        LeafWailing {
            leaf_wail: self,
            frame: frame,
            offset: offset,
            patch_id: patch_id,
        }
    }
}
