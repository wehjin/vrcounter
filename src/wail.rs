extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use star::Star;
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
pub struct Wail {
    color: [f32; 4],
    frame: Frame,
}

#[derive(Clone, Debug)]
pub struct WailModel {
    offset: Offset,
    patch_id: u64,
}

impl Wail {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        Wail { color: color, frame: frame }
    }
}

impl Star for Wail {
    type Mdl = WailModel;
    type Msg = WailIn;
    type Out = WailOut;

    fn init(&self) -> WailModel {
        let patch_id = rand::random::<u64>();
        let offset = Offset::default();
        WailModel { offset: offset, patch_id: patch_id }
    }
    fn view(&self, model: &WailModel) -> Vision<WailIn> {
        let cage = Cage::from((self.frame, model.offset));
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, model.patch_id);
        let mut vision = Vision::new();
        vision.add_patch(patch);
        vision
    }
    fn update(&self, model: &WailModel, message: &WailIn) -> WailModel {
        let mut new_model = (*model).clone();
        match message {
            &WailIn::Offset(offset) => {
                new_model.offset = offset;
            }
        }
        new_model
    }
}
