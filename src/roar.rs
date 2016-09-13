pub mod demo {
    use vision::Vision;
    use patch::{Sigil, Patch};
    use beat::Beat;
    use std::time::{Instant, Duration};
    use common::Wish;
    use star::Star;

    #[derive(Clone)]
    pub struct Model {
        pub index: usize,
        pub end_instant: Instant,
    }

    #[derive(Clone)]
    pub enum Message {
        Ignore,
        IncrementIndex,
    }

    pub fn from(colors: Vec<[f32; 4]>) -> RainbowStar {
        RainbowStar {
            colors: colors.clone()
        }
    }

    #[derive(Clone)]
    pub struct RainbowStar {
        colors: Vec<[f32; 4]>
    }

    impl Star for RainbowStar {
        type Mdl = Model;
        type Msg = Message;
        type Out = ();

        fn init(&self) -> (Self::Mdl, Vec<Wish>) {
            let model = Model {
                index: 0,
                end_instant: Instant::now() + Duration::from_secs(30),
            };
            (model, vec![])
        }

        fn update(&self, message: Self::Msg, model: &Self::Mdl) -> (Option<Self::Mdl>, Vec<Wish>, Vec<Self::Out>) {
            if let Message::IncrementIndex = message {
                let next_index = (model.index + 1) % self.colors.len();
                let next_model = Model { index: next_index, end_instant: model.end_instant };
                (Some(next_model), vec![], vec![])
            } else {
                (None, vec![], vec![])
            }
        }

        fn view(&self, model: &Self::Mdl) -> Vision<Self::Msg> {
            let mut vision = Vision::new(|wish| match wish {
                Wish::Tick => Some(Message::IncrementIndex),
                _ => None,
            });
            let patch = Patch::new(15674u64, 0.55, 0.65, -0.35, -0.25, 0.25, self.colors[model.index].clone(), Sigil::Fill);
            vision.add_patch(patch);
            let beat = Beat::until_instant(24352u64, model.end_instant);
            vision.add_beat(beat);
            vision
        }
    }
}
