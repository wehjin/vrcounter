use std::rc::Rc;
use summoner::{Report};
use vision::Vision;

pub struct Roar<Mod: Clone, Msg, Out> {
    pub init: Rc<Fn() -> Mod>,
    pub update: Rc<Fn(Msg, &Mod) -> Report<Mod, Out>>,
    pub view: Rc<Fn(&Mod) -> Vision<Msg>>,
}

impl<Mod: Clone, Msg, Out> Roar<Mod, Msg, Out> {
    pub fn create(
        init: Rc<Fn() -> Mod>,
        update: Rc<Fn(Msg, &Mod) -> Report<Mod, Out>>,
        view: Rc<Fn(&Mod) -> Vision<Msg>>
    ) -> Self {
        Roar { init: init, update: update, view: view }
    }
}

pub mod color {
    use super::*;
    use vision::{Vision, VisionMessage};
    use summoner::{Report};
    use patch::{Sigil, Patch};
    use beat::Beat;
    use std::rc::Rc;
    use std::time::{Instant, Duration};

    #[derive(Clone)]
    pub struct Model {
        pub colors: Rc<Vec<[f32; 4]>>,
        pub index: usize,
        pub end_instant: Instant,
    }

    #[derive(Clone)]
    pub enum Message {
        IncrementIndex,
    }

    #[derive(Clone)]
    pub enum Outcome {
        Done,
    }

    pub fn from(colors: Vec<[f32; 4]>) -> Roar<Model, Message, Outcome> {
        let init_colors = Rc::new(colors);
        let update_colors = init_colors.clone();

        let init = move || -> Model {
            Model {
                colors: init_colors.clone(),
                index: 0,
                end_instant: Instant::now() + Duration::from_secs(30),
            }
        };
        let update = move |message: Message, model: &Model| -> Report<Model, Outcome> {
            match message {
                Message::IncrementIndex => {
                    let next_index = (model.index + 1) % (*update_colors).len();
                    Report::Model(Model {
                        colors: update_colors.clone(),
                        index: next_index,
                        end_instant: model.end_instant,
                    })
                }
            }
        };
        let view = move |model: &Model| -> Vision<Message> {
            let mut vision = Vision::new(
                Rc::new(move |vision_message: VisionMessage| -> Message {
                    match vision_message {
                        VisionMessage::Tick => Message::IncrementIndex,
                    }
                }));

            let patch = Patch::new(15674u64, 0.55, 0.65, -0.35, -0.25, 0.25, model.colors[model.index].clone(), Sigil::Fill);
            vision.add_patch(patch);

            let beat = Beat::until_instant(24352u64, model.end_instant);
            vision.add_beat(beat);

            vision
        };
        Roar::create(Rc::new(init), Rc::new(update), Rc::new(view))
    }
}
