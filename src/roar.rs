use std::rc::Rc;
use vision::Vision;
use common::Report;

pub struct Roar<Mdl, Msg, Out> where Mdl: Clone {
    pub init: Rc<Fn() -> Mdl>,
    pub update: Rc<Fn(Msg, &Mdl) -> Report<Mdl, Out>>,
    pub view: Rc<Fn(&Mdl) -> Vision<Msg>>,
}

impl<Mdl, Msg, Out> Roar<Mdl, Msg, Out> where Mdl: Clone {
    pub fn create<F, G, H>(init: F, update: G, view: H) -> Self where F: Fn() -> Mdl + 'static,
                                                                      G: Fn(Msg, &Mdl) -> Report<Mdl, Out> + 'static,
                                                                      H: Fn(&Mdl) -> Vision<Msg> + 'static {
        Roar { init: Rc::new(init), update: Rc::new(update), view: Rc::new(view) }
    }
}

pub mod demo {
    use super::*;
    use vision::Vision;
    use patch::{Sigil, Patch};
    use beat::Beat;
    use std::time::{Instant, Duration};
    use common::{Report, Wish};

    #[derive(Clone)]
    pub struct Model {
        pub colors: Vec<[f32; 4]>,
        pub index: usize,
        pub end_instant: Instant,
    }

    #[derive(Clone)]
    pub enum Message {
        IncrementIndex,
    }

    pub fn from(colors: Vec<[f32; 4]>) -> Roar<Model, Message, ()> {
        let init_colors = colors.clone();
        let update_colors = colors.clone();
        Roar::create(
            move || Model {
                colors: init_colors.clone(),
                index: 0,
                end_instant: Instant::now() + Duration::from_secs(30),
            },
            move |Message::IncrementIndex, model| {
                let next_index = (model.index + 1) % update_colors.len();
                Report::Model(Model {
                    colors: update_colors.clone(),
                    index: next_index,
                    end_instant: model.end_instant,
                })
            },
            |model| {
                let mut vision = Vision::create(|vision_message| match vision_message {
                    Wish::Tick => Message::IncrementIndex,
                });
                let patch = Patch::new(15674u64, 0.55, 0.65, -0.35, -0.25, 0.25, model.colors[model.index].clone(), Sigil::Fill);
                vision.add_patch(patch);
                let beat = Beat::until_instant(24352u64, model.end_instant);
                vision.add_beat(beat);
                vision
            })
    }
}
