extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::*;
use vrcounter::color::*;
use std::sync::Arc;
use std::rc::Rc;
use rand::random;
use cage::{Frame};

#[derive(Clone, Debug)]
struct In;

#[derive(Clone, Debug)]
struct Out;

#[derive(Clone, Debug)]
struct App;

#[derive(Clone, Debug)]
struct Model {
    patch_id: u64,
    beat_id: u64,
    wailing: Rc<Wailing<u32>>,
}

impl Star for App {
    type Mdl = Model;
    type Msg = In;
    type Out = Out;

    fn init(&self) -> Model {
        let frame = Frame::from((0.20, 0.20, 0.20));
        let leaf1 = LeafWailer::new(CYAN, frame) as LeafWailer<u32>;
        let leaf2 = LeafWailer::new(MAGENTA, frame) as LeafWailer<u32>;
        let leaf3 = LeafWailer::new(YELLOW, frame) as LeafWailer<u32>;
        let expand1 = leaf1.expand_right(leaf2) as ExpandRightWailer<u32, u32, u32>;
        let wail = expand1.expand_right(leaf3) as ExpandRightWailer<u32, u32, u32>;
        let wailing = wail.summon();
        Model {
            patch_id: random::<u64>(),
            beat_id: random::<u64>(),
            wailing: Rc::new(wailing)
        }
    }

    fn view(&self, model: &Model) -> Vision<In> {
        let mut vision = Vision::new();
        let wail_vision = model.wailing.as_ref().view();
        vision.add_vision(wail_vision, |_| None);
        vision
    }

    fn update(&self, model: &Model, _: &In) -> Model {
        model.clone()
    }
}


fn main() {
    let star_builder = Arc::new(move || App);
    vrcounter::start(star_builder)
}
