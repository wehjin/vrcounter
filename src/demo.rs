extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::IdSource;
use vrcounter::Summoner;
use vrcounter::Wish;
use vrcounter::Vision;
use vrcounter::{Star, Substar};
use vrcounter::Hand;
use std::sync::Arc;
use std::rc::Rc;
use cage::Cage;
use cage::Offset;
use vrcounter::{howl, scream, roar};
use vrcounter::color::*;
use vrcounter::Sigil;
use vrcounter::Patch;
use vrcounter::Mist;
use vrcounter::roar::RainbowStar;
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
    substar: Substar<RainbowStar>,
    component_composite_substar: ComponentCompositeSubstar,
}

#[derive(Clone)]
pub enum Message {
    SeeHand(Hand),
    ForwardToRainbow(roar::Message),
}

#[derive(Clone, Debug)]
pub enum ComponentStar {
    Scream(scream::Scream, < scream::Scream as Star >::Msg),
    Howl(howl::Howl),
    Misty(howl::MistyStar)
}

#[derive(Clone, Debug)]
pub enum ComponentMessage {
    Scream(< scream::Scream as Star >::Msg),
    Howl(< howl::Howl as Star >::Msg),
    Misty(< howl::MistyStar as Star >::Msg),
}

#[derive(Clone, Debug)]
enum ComponentSubstar {
    Scream(Substar<scream::Scream>),
    Howl(Substar<howl::Howl>),
    Misty(Substar<howl::MistyStar>)
}

#[derive(Clone, Debug)]
pub struct ComponentCompositeSubstar {
    component_substars: Vec<ComponentSubstar>
}

impl ComponentCompositeSubstar {
    fn init(stars: Vec<ComponentStar>) -> Self {
        let mut component_substars = Vec::new();
        for component in stars {
            let component_substar = match component {
                ComponentStar::Scream(scream, message) => {
                    ComponentSubstar::Scream(Substar::init(Rc::new(scream)).update(message).unwrap())
                },
                ComponentStar::Howl(howl) => {
                    ComponentSubstar::Howl(Substar::init(Rc::new(howl)))
                },
                ComponentStar::Misty(misty) => {
                    ComponentSubstar::Misty(Substar::init(Rc::new(misty)))
                }
            };
            component_substars.push(component_substar);
        }
        ComponentCompositeSubstar {
            component_substars: component_substars
        }
    }
    fn view(&self) -> Vision<()> {
        let mut vision = Vision::new();
        for substars in &self.component_substars {
            match substars {
                &ComponentSubstar::Scream(ref substar) => vision.add_vision(substar.view(), |_| None),
                &ComponentSubstar::Howl(ref substar) => vision.add_vision(substar.view(), |_| None),
                &ComponentSubstar::Misty(ref substar) => vision.add_vision(substar.view(), |_| None),
            }
        };
        vision
    }
    fn update(&self, _: ComponentMessage) -> Option<Self> {
        None
    }
}


impl Star for MyStar {
    type Mdl = Model;
    type Msg = Message;
    type Out = Outcome;

    fn init(&self) -> Model {
        Model {
            colors: [BLUE, YELLOW],
            color_index: 0,
            mist_id: rand::random::<u64>(),
            patch_id: rand::random::<u64>(),
            delta_z_option: None,
            cage: Cage::from((-0.70, -0.50, -0.10, 0.10, 0.00, 0.20)),
            substar: Substar::init(Rc::new(roar::from(vec![GREEN, RED, BLUE, CYAN, MAGENTA, YELLOW]))),
            component_composite_substar: ComponentCompositeSubstar::init(vec![
                ComponentStar::Scream(scream::new(rand::random::<u64>(), CYAN), scream::Message::FitToCage(Cage::from((-0.3, -0.2, -0.25, -0.15, 0.03, 0.03)))),
                ComponentStar::Scream(scream::new(rand::random::<u64>(), MAGENTA), scream::Message::FitToCage(Cage::from((-0.4, -0.3, -0.25, -0.15, 0.03, 0.03)))),
                ComponentStar::Scream(scream::new(rand::random::<u64>(), YELLOW), scream::Message::FitToCage(Cage::from((-0.5, -0.4, -0.25, -0.15, 0.03, 0.03)))),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), RED, Cage::from((-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)), Sigil::Fill)),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), GREEN, Cage::from((0.25, 0.75, 0.0, 0.5, -0.01, -0.01)), Sigil::Fill)),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), CYAN, Cage::from((-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('J'))),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), YELLOW, Cage::from((0.00, 0.06, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('y'))),
                ComponentStar::Misty(howl::misty(rand::random::<u64>(), Default::default())),
            ]),
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
        vision.add_vision(model.substar.view(), |x| Some(Message::ForwardToRainbow(x)));
        vision.add_vision(model.component_composite_substar.view(), |_| None);
        vision
    }

    fn update(&self, model: &Model, message: Message) -> Option<Model> {
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
                    Message::SeeHand(hand) => self.see_hand(current_model, hand),
                    Message::ForwardToRainbow(submessage) => self.forward_to_rainbow(current_model, submessage),
                }
            };
            if next_model_op.is_some() {
                current_model_op = next_model_op;
            };
        }
        current_model_op
    }
}

impl MyStar {
    fn forward_to_rainbow(&self, model: &Model, submessage: roar::Message) -> Option<Model> {
        if let Some(new_substar) = model.substar.update(submessage) {
            let mut new_model = model.clone();
            new_model.substar = new_substar;
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

fn main() {
    let star_builder = Arc::new(|| MyStar);
    vrcounter::start(star_builder)
}
