extern crate vrcounter;
extern crate cage;
extern crate rand;

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
use std::collections::VecDeque;


#[derive(Clone, Debug)]
pub enum ComponentStar {
    Scream(scream::Scream),
    Howl(howl::Howl),
    Misty(howl::MistyStar),
    Rainbow(roar::RainbowStar),
}

#[derive(Clone, Debug)]
pub enum ComponentMessage {
    Scream(u32, < scream::Scream as Star >::Msg),
    Howl(u32, < howl::Howl as Star >::Msg),
    Misty(u32, < howl::MistyStar as Star >::Msg),
    Rainbow(u32, < roar::RainbowStar as Star >::Msg),
}

#[derive(Clone, Debug)]
enum ComponentSubstar {
    Scream(u32, Substar<scream::Scream>),
    Howl(u32, Substar<howl::Howl>),
    Misty(u32, Substar<howl::MistyStar>),
    Rainbow(u32, Substar<roar::RainbowStar>)
}

#[derive(Clone, Debug)]
pub struct ComponentCompositeSubstar {
    component_substars: Vec<ComponentSubstar>
}

impl ComponentCompositeSubstar {
    fn init(stars: Vec<ComponentStar>) -> Self {
        let mut component_substars = Vec::new();
        let mut index = 0;
        for component in stars {
            let component_substar = match component {
                ComponentStar::Scream(star) => {
                    ComponentSubstar::Scream(index, Substar::init(Rc::new(star)))
                },
                ComponentStar::Howl(star) => {
                    ComponentSubstar::Howl(index, Substar::init(Rc::new(star)))
                },
                ComponentStar::Misty(star) => {
                    ComponentSubstar::Misty(index, Substar::init(Rc::new(star)))
                },
                ComponentStar::Rainbow(star) => {
                    ComponentSubstar::Rainbow(index, Substar::init(Rc::new(star)))
                }
            };
            component_substars.push(component_substar);
            index += 1;
        }
        ComponentCompositeSubstar {
            component_substars: component_substars
        }
    }
    fn view(&self) -> Vision<ComponentMessage> {
        let mut vision = Vision::new();
        for substars in &self.component_substars {
            match substars {
                &ComponentSubstar::Scream(index, ref substar) => {
                    vision.add_vision(substar.view(), move |x| Some(ComponentMessage::Scream(index, x)))
                },
                &ComponentSubstar::Howl(index, ref substar) => {
                    vision.add_vision(substar.view(), move |x| Some(ComponentMessage::Howl(index, x)))
                },
                &ComponentSubstar::Misty(index, ref substar) => {
                    vision.add_vision(substar.view(), move |x| Some(ComponentMessage::Misty(index, x)))
                },
                &ComponentSubstar::Rainbow(index, ref substar) => {
                    vision.add_vision(substar.view(), move |x| Some(ComponentMessage::Rainbow(index, x)))
                },
            }
        };
        vision
    }
    fn update(&self, message: &ComponentMessage) -> Option<Self> {
        let mut new_component_substars = Vec::new();
        let mut updates = 0;
        for component_substar in &self.component_substars {
            if let Some(new_component_substar) = match component_substar {
                &ComponentSubstar::Scream(ref index, ref substar) => {
                    if let &ComponentMessage::Scream(ref target_index, ref submessage) = message {
                        if target_index == index {
                            if let Some(new_substar) = substar.update(submessage) {
                                Some(ComponentSubstar::Scream(*index, new_substar))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                &ComponentSubstar::Howl(ref index, ref substar) => {
                    if let &ComponentMessage::Howl(ref target_index, ref submessage) = message {
                        if target_index == index {
                            if let Some(new_substar) = substar.update(submessage) {
                                Some(ComponentSubstar::Howl(*index, new_substar))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                &ComponentSubstar::Misty(ref index, ref substar) => {
                    if let &ComponentMessage::Misty(ref target_index, ref submessage) = message {
                        if target_index == index {
                            if let Some(new_substar) = substar.update(submessage) {
                                Some(ComponentSubstar::Misty(*index, new_substar))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                &ComponentSubstar::Rainbow(ref index, ref substar) => {
                    if let &ComponentMessage::Rainbow(ref target_index, ref submessage) = message {
                        if target_index == index {
                            if let Some(new_substar) = substar.update(submessage) {
                                Some(ComponentSubstar::Rainbow(*index, new_substar))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
            } {
                new_component_substars.push(new_component_substar);
                updates += 1;
            } else {
                new_component_substars.push(component_substar.clone());
            }
        };
        if updates > 0 {
            Some(ComponentCompositeSubstar { component_substars: new_component_substars })
        } else {
            None
        }
    }
}


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
    composite_substar: ComponentCompositeSubstar,
}

#[derive(Clone)]
pub enum Message {
    SeeHand(Hand),
    ForwardToComponent(ComponentMessage),
}

impl Star for MyStar {
    type Mdl = Model;
    type Msg = Message;
    type Out = Outcome;

    fn init(&self) -> Model {
        let mut model = Model {
            colors: [BLUE, YELLOW],
            color_index: 0,
            mist_id: rand::random::<u64>(),
            patch_id: rand::random::<u64>(),
            delta_z_option: None,
            cage: Cage::from((-0.70, -0.50, -0.10, 0.10, 0.00, 0.20)),
            composite_substar: ComponentCompositeSubstar::init(vec![
                ComponentStar::Scream(scream::new(rand::random::<u64>(), CYAN)),
                ComponentStar::Scream(scream::new(rand::random::<u64>(), MAGENTA)),
                ComponentStar::Scream(scream::new(rand::random::<u64>(), YELLOW)),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), RED, Cage::from((-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)), Sigil::Fill)),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), GREEN, Cage::from((0.25, 0.75, 0.0, 0.5, -0.01, -0.01)), Sigil::Fill)),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), CYAN, Cage::from((-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('J'))),
                ComponentStar::Howl(howl::new(rand::random::<u64>(), YELLOW, Cage::from((0.00, 0.06, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('y'))),
                ComponentStar::Misty(howl::misty(rand::random::<u64>(), Default::default())),
                ComponentStar::Rainbow(roar::from(vec![GREEN, RED, BLUE, CYAN, MAGENTA, YELLOW])),
            ]),
        };
        let updates = vec![
            ComponentMessage::Scream(0, scream::Message::FitToCage(Cage::from((-0.3, -0.2, -0.25, -0.15, 0.03, 0.03)))),
            ComponentMessage::Scream(1, scream::Message::FitToCage(Cage::from((-0.4, -0.3, -0.25, -0.15, 0.03, 0.03)))),
            ComponentMessage::Scream(2, scream::Message::FitToCage(Cage::from((-0.5, -0.4, -0.25, -0.15, 0.03, 0.03)))),
        ];
        for update in &updates {
            let message = Message::ForwardToComponent(update.clone());
            if let Some(new_model) = self.update(&model, &message) {
                model = new_model;
            }
        }
        model
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
        vision.add_vision(model.composite_substar.view(), |x| Some(Message::ForwardToComponent(x)));
        vision
    }

    fn update(&self, model: &Model, message: &Message) -> Option<Model> {
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
                    &Message::SeeHand(hand) => self.see_hand(current_model, hand),
                    &Message::ForwardToComponent(ref component_submessage) => self.forward_to_component(current_model, component_submessage),
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
    fn forward_to_component(&self, model: &Model, submessage: &ComponentMessage) -> Option<Model> {
        if let Some(new_composite_substar) = model.composite_substar.update(submessage) {
            let mut new_model = model.clone();
            new_model.composite_substar = new_composite_substar;
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
        Some(new_model)
    }
}

fn main() {
    let star_builder = Arc::new(|| MyStar);
    vrcounter::start(star_builder)
}
