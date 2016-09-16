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
use vrcounter::{howl, scream, roar};
use vrcounter::color::*;
use vrcounter::Sigil;
use vrcounter::Patch;
use vrcounter::Mist;
use vrcounter::roar::demo::RainbowStar;
use std::collections::VecDeque;

#[derive(Clone)]
struct MyStar;

#[derive(Clone)]
pub struct Outcome;

#[derive(Clone, Debug)]
pub struct Model {
    pub colors: [[f32; 4]; 2],
    pub color_index: usize,
    pub mist_id: u64,
    pub patch_id: u64,
    pub delta_z_option: Option<f32>,
    pub cage: Cage,
    pub rainbow_star: RainbowStar,
    pub rainbow_star_model: roar::demo::Model,
}

#[derive(Clone)]
pub enum Message {
    SeeHand(Hand),
    ForwardToRainbow(roar::demo::Message)
}

impl MyStar {
    fn forward_to_rainbow(&self, model: &Model, submessage: roar::demo::Message) -> Option<Model> {
        let mut well = Well::new(|_| None) as Well<(), Message>;
        if let Some(new_submodel) = model.rainbow_star.update(&model.rainbow_star_model, submessage, &mut well) {
            let mut new_model = model.clone();
            new_model.rainbow_star_model = new_submodel;
            Some(new_model)
        } else {
            None
        }
    }
    fn see_hand(&self, model: &Model, hand: Hand) -> Option<Model> {
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
    }
}

impl Star for MyStar {
    type Mdl = Model;
    type Msg = Message;
    type Out = Outcome;

    fn init(&self) -> Model {
        let patch_id = rand::random::<u64>();
        let mist_id = rand::random::<u64>();
        let cage = Cage::from((-0.70, -0.50, -0.10, 0.10, 0.00, 0.20));
        let rainbow_star = roar::demo::from(vec![GREEN, RED, BLUE, CYAN, MAGENTA, YELLOW]);
        Model {
            colors: [BLUE, YELLOW],
            color_index: 0,
            mist_id: mist_id,
            patch_id: patch_id,
            delta_z_option: None,
            cage: cage,
            rainbow_star_model: rainbow_star.init(),
            rainbow_star: rainbow_star,
        }
    }

    fn view(&self, model: &Model) -> Vision<Message> {
        let mut vision = Vision::new();
        let color = model.colors[model.color_index % model.colors.len()];
        vision.add_patch(Patch::from_cage(&model.cage, color, Sigil::Fill, model.patch_id));
        vision.add_mist(Mist::new(model.mist_id, model.cage), |wish| {
            if let Wish::SenseHand(hand) = wish {
                Some(Message::SeeHand(hand))
            } else {
                None
            }
        });
        vision.add_vision(model.rainbow_star.view(&model.rainbow_star_model), |x| Some(Message::ForwardToRainbow(x)));
        vision
    }

    fn update<T>(&self, model: &Model, message: Message, well: &mut Well<Outcome, T>) -> Option<Model> {
        let mut deque = VecDeque::new();
        deque.push_back(message);
        let mut current_model_op = None;
        while let Some(message) = deque.pop_front() {
            let next_model_op = {
                let current_model = match current_model_op {
                    Some(ref model) => model,
                    None => model,
                };
                match message {
                    Message::SeeHand(hand) => self.see_hand(model, hand),
                    Message::ForwardToRainbow(submessage) => self.forward_to_rainbow(model, submessage),
                }
            };
            if next_model_op.is_some() {
                current_model_op = next_model_op;
            }
        }
        current_model_op
    }
}

fn summon(id_source: &mut IdSource, summoner: &mut Summoner) {
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
