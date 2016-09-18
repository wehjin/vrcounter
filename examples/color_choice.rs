extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::*;
use vrcounter::color::*;
use std::sync::Arc;
use rand::random;
use std::time::{Instant, Duration};

#[derive(Clone, Debug)]
struct Model {
    patch_id: u64,
    beat_id: u64,
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
        Model { patch_id: random::<u64>(), beat_id: random::<u64>() }
    }

    fn view(&self, model: &Model) -> Vision<In> {
        let mut vision = Vision::new();
        vision.add_patch(Patch::new(model.patch_id, -0.5, 0.5, -0.5, 0.5, 0.0, BLUE, Sigil::Fill));
        vision.add_beat(Beat::until_instant(model.beat_id, Instant::now() + Duration::from_secs(60)), |_|{
            println!("In adapter");
            None
        });
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
