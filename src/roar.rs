use vision::Vision;
use patch::{Sigil, Patch};
use beat::Beat;
use std::time::{Instant, Duration};
use common::Wish;
use star::Star;

#[derive(Clone, Debug)]
pub struct Model {
    pub index: usize,
    pub end_instant: Instant,
}

#[derive(Clone, Debug)]
pub enum Message {
    IncrementIndex,
}

#[derive(Clone)]
pub struct Outcome;

pub fn from(colors: Vec<[f32; 4]>) -> RainbowStar {
    RainbowStar {
        colors: colors.clone()
    }
}

#[derive(Clone, Debug)]
pub struct RainbowStar {
    colors: Vec<[f32; 4]>
}

impl Star for RainbowStar {
    type Mdl = Model;
    type Msg = Message;
    type Out = Outcome;

    fn init(&self) -> Model {
        Model {
            index: 0,
            end_instant: Instant::now() + Duration::from_secs(30),
        }
    }

    fn update(&self, model: &Model, message: &Message) -> Model {
        match message {
            &Message::IncrementIndex => {
                let next_index = (model.index + 1) % self.colors.len();
                Model { index: next_index, end_instant: model.end_instant }
            }
        }
    }

    fn view(&self, model: &Self::Mdl) -> Vision<Self::Msg> {
        let mut vision = Vision::new();
        let patch = Patch::new(15674u64, 0.55, 0.65, -0.35, -0.25, 0.25, self.colors[model.index].clone(), Sigil::Fill);
        vision.add_patch(patch);
        let beat = Beat::until_instant(24352u64, model.end_instant);
        vision.add_beat(beat, |wish| match wish {
            Wish::Tick => Some(Message::IncrementIndex),
            _ => None,
        });
        vision
    }
}

