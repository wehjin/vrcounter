extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::IdSource;
use vrcounter::Summoner;
use vrcounter::Wish;
use vrcounter::Vision;
use vrcounter::Well;
use vrcounter::Star;
use vrcounter::Hand;
use std::sync::Arc;
use cage::Cage;
use cage::Offset;

#[derive(Clone, Debug)]
pub struct Model {
    pub colors: [[f32; 4]; 2],
    pub color_index: usize,
    pub mist_id: u64,
    pub patch_id: u64,
    pub delta_z_option: Option<f32>,
    pub cage: Cage,
}

#[derive(Clone)]
pub enum Message {
    Ignore,
    Toggle,
    SeeHand(Hand),
}

#[derive(Clone)]
pub struct Outcome;

#[derive(Clone)]
struct MyStar;

impl Star for MyStar {
    type Mdl = Model;
    type Msg = Message;
    type Out = Outcome;

    fn init(&self) -> Model {
        use std::rc::Rc;
        use vrcounter::color::{BLUE, YELLOW};

        let patch_id = rand::random::<u64>();
        let mist_id = rand::random::<u64>();
        let cage = Cage::from((-0.70, -0.50, -0.10, 0.10, 0.00, 0.20));
        Model {
            colors: [BLUE, YELLOW],
            color_index: 0,
            mist_id: mist_id,
            patch_id: patch_id,
            delta_z_option: None,
            cage: cage
        }
    }

    fn update<T>(&self, model: &Model, message: Message, well: &mut Well<Outcome, T>) -> Option<Model> {
        if let Message::SeeHand(hand) = message {
            let Offset { x, y, z } = hand.offset;
            let mut new_delta_z_option = None;
            let mut did_toggle = false;
            if model.cage.contains(x, y, z) {
                let (_, _, _, _, f, n) = model.cage.limits();
                let center_z = (f + n) / 2.0;
                let delta_z = z - center_z;
                new_delta_z_option = Some(delta_z);
                if let Some(previous_delta_z) = model.delta_z_option {
                    const BIAS: f32 = 0.045;
                    if delta_z < BIAS && previous_delta_z >= BIAS {
                        println!("See Toggle!");
                        did_toggle = true;
                    }
                }
            }
            let mut new_model = model.clone();
            if did_toggle {
                new_model.color_index = model.color_index + 1;
            }
            new_model.delta_z_option = new_delta_z_option;
            // TODO: Deal with well.
            Some(new_model)
        } else {
            None
        }
    }

    fn view(&self, model: &Model) -> Vision<Message> {
        use vrcounter::{Patch, Sigil};
        use vrcounter::Mist;
        let mut vision = Vision::new(|wish| {
            if let Wish::SenseHand(hand) = wish {
                Some(Message::SeeHand(hand))
            } else {
                None
            }
        });
        let color = model.colors[model.color_index % model.colors.len()];
        vision.add_patch(Patch::from_cage(&model.cage, color, Sigil::Fill, model.patch_id));
        vision.add_mist(Mist::new(model.mist_id, model.cage));
        vision
    }
}

fn summon(id_source: &mut IdSource, summoner: &mut Summoner) {
    use cage::Cage;
    use vrcounter::{howl, scream, roar};
    use vrcounter::color::*;
    use vrcounter::Sigil;

    let roar = roar::demo::from(vec![GREEN, RED, BLUE, CYAN, MAGENTA, YELLOW]);
    summoner.summon(id_source, &roar);

    let scream_id1 = id_source.id();
    let scream1 = scream::from_color(scream_id1, CYAN);
    let screaming1 = summoner.summon(id_source, &scream1);
    let cage1 = Cage::from((-0.3, -0.2, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming1, Wish::FitToCage(Cage::from(cage1)));

    let scream_id2 = id_source.id();
    let scream2 = scream::from_color(scream_id2, MAGENTA);
    let screaming2 = summoner.summon(id_source, &scream2);
    let cage2 = Cage::from((-0.4, -0.3, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming2, Wish::FitToCage(Cage::from(cage2)));
    let scream_id3 = id_source.id();
    let screaming3 = summoner.summon(id_source, &scream::from_color(scream_id3, YELLOW));
    let cage3 = Cage::from((-0.5, -0.4, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming3, Wish::FitToCage(Cage::from(cage3)));
    let howls = vec![
        howl::create(id_source.id(), RED, Cage::from((-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)), Sigil::Fill),
        howl::create(id_source.id(), GREEN, Cage::from((0.25, 0.75, 0.0, 0.5, -0.01, -0.01)), Sigil::Fill),
        howl::create(id_source.id(), CYAN, Cage::from((-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('J')),
        howl::create(id_source.id(), YELLOW, Cage::from((0.00, 0.06, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('y')),
    ];
    for howl in &howls {
        summoner.summon(id_source, howl);
    }
    let howl_id = id_source.id();
    summoner.summon(id_source, &howl::misty(howl_id, Default::default()));
}

fn main() {
    let star_builder = Arc::new(|| MyStar);
    vrcounter::start(star_builder)
}
