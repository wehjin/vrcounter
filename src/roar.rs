pub mod demo {
    use vision::Vision;
    use patch::{Sigil, Patch};
    use beat::Beat;
    use std::time::{Instant, Duration};
    use common::Wish;
    use report::Report;
    use star::SeedStar;

    #[derive(Clone)]
    pub struct Model {
        pub colors: Vec<[f32; 4]>,
        pub index: usize,
        pub end_instant: Instant,
    }

    #[derive(Clone)]
    pub enum Message {
        Ignore,
        IncrementIndex,
    }

    pub fn from(colors: Vec<[f32; 4]>) -> SeedStar<Model, Message, ()> {
        let init_colors = colors.clone();
        let update_colors = colors.clone();
        SeedStar::create(
            move || {
                let model = Model {
                    colors: init_colors.clone(),
                    index: 0,
                    end_instant: Instant::now() + Duration::from_secs(30),
                };
                (model, vec![])
            },
            move |message, model| match message {
                Message::IncrementIndex => {
                    let next_index = (model.index + 1) % update_colors.len();
                    Report::Model(Model {
                        colors: update_colors.clone(),
                        index: next_index,
                        end_instant: model.end_instant,
                    })
                }
                _ => Report::Unchanged
            },
            |model| {
                let mut vision = Vision::create(|wish| match wish {
                    Wish::Tick => Message::IncrementIndex,
                    _ => Message::Ignore
                });
                let patch = Patch::new(15674u64, 0.55, 0.65, -0.35, -0.25, 0.25, model.colors[model.index].clone(), Sigil::Fill);
                vision.add_patch(patch);
                let beat = Beat::until_instant(24352u64, model.end_instant);
                vision.add_beat(beat);
                vision
            })
    }
}
