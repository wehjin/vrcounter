extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::*;
use vrcounter::color::*;
use std::sync::Arc;
use rand::random;
use cage::{Frame};

#[derive(Clone, Debug)]
struct Model {
    patch_id: u64,
    beat_id: u64,
    wail: Wail,
    wail_model: WailModel,
}

#[derive(Clone, Debug)]
struct In;

#[derive(Clone, Debug)]
struct Out;

#[derive(Clone, Debug)]
struct App;

impl Star for App {
    type Mdl = Model;
    type Msg = In;
    type Out = Out;

    fn init(&self) -> Model {
        let wail = Wail::new(CYAN, Frame::from((0.20, 0.20, 0.20)));
        let wail_model = wail.init();
        Model {
            patch_id: random::<u64>(),
            beat_id: random::<u64>(),
            wail: wail,
            wail_model: wail_model
        }
    }

    fn view(&self, model: &Model) -> Vision<In> {
        let mut vision = Vision::new();
        let wail_vision = model.wail.view(&model.wail_model);
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
