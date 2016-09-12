extern crate cage;

use viewer::{Viewer};
use common::{IdSource};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;
use summoner::Summoner;
use demon::DemonVision;
use std::boxed::Box;
use hand::Hand;
use common::Wish;
use std::sync::Arc;
use star::Star;

pub enum Message {
    Ignore,
    Stop,
    EmitAnimationFrame,
    SetHand(Hand),
}

struct Model {
    summoner: Summoner,
    id_source: IdSource,
}

fn init() -> Model {
    Model {
        summoner: Summoner::new(),
        id_source: IdSource::new(),
    }
}

fn update(message: Message, mut model: Model) -> Option<Model> {
    match message {
        Message::Ignore => Some(model),
        Message::Stop => None,
        Message::EmitAnimationFrame => {
            (&mut model.summoner).update(Wish::Tick);
            Some(model)
        },
        Message::SetHand(hand) => {
            // TODO Maybe check if hand occupies mist
            (&mut model.summoner).update(Wish::SenseHand(hand));
            Some(model)
        }
    }
}

fn view(model: &Model, viewer: &Viewer) {
    use patch::Patch;
    use mist::Mist;
    viewer.clear();
    let demon_boxes = model.summoner.get_demon_boxes();
    for demon_box in demon_boxes {
        let vision_box: Box<DemonVision> = (&demon_box).see();
        let patches: &HashMap<u64, Patch> = (*vision_box).patches();
        for (_, patch) in patches.iter() {
            viewer.add_patch(*patch);
        }
        let mists: &HashMap<u64, Mist> = (*vision_box).mists();
        for (_, mist) in mists.iter() {
            viewer.add_mist(*mist);
        }
    }
}

pub fn start<S: Star, F>(viewer: Viewer, star_builder: Arc<F>)
    -> Sender<Message> where S: 'static,
                             F: Fn() -> S + Send + Sync + 'static
{
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut model = init();
        let star = star_builder();
        model.summoner.summon(&mut model.id_source, &star, |_| Message::Ignore);
        view(&model, &viewer);
        loop {
            match rx.recv() {
                Ok(message) => {
                    match update(message, model) {
                        Option::None => { break; },
                        Option::Some(next_model) => {
                            model = next_model;
                            view(&model, &viewer);
                        }
                    }
                }
                Err(err) => {
                    println!("ERROR: {:?}", err);
                    break;
                },
            };
        }
    });
    tx
}

pub fn stop(agent: Sender<Message>) {
    agent.send(Message::Stop).unwrap_or(())
}

