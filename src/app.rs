extern crate cage;

use color::{GREEN, RED, BLUE, CYAN, YELLOW, MAGENTA};
use viewer::{Viewer};
use common::{IdSource};
use scream;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;
use summoner::Summoner;
use demon::DemonVision;
use roar;
use color;
use std::boxed::Box;
use cage::Cage;
use patch::Sigil;
use hand::Hand;
use common::Wish;

pub enum Message {
    Stop,
    EmitAnimationFrame,
    SetHand(Hand),
}

struct Model {
    summoner: Summoner,
}

pub enum Outcome {
    Done,
}

fn init() -> Model {
    use howl;

    let mut id_source = IdSource::new();

    let mut summoner = Summoner::new();
    let roar = roar::demo::from(vec![color::GREEN, color::RED, color::BLUE, color::CYAN, color::MAGENTA, color::YELLOW]);
    summoner.summon(&mut id_source, &roar, |_| Outcome::Done);

    let scream_id1 = id_source.id();
    let screaming1 = summoner.summon(&mut id_source, &scream::from_color(scream_id1, CYAN), |_| Outcome::Done);
    let cage1 = Cage::from((-0.3, -0.2, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming1, Wish::FitToCage(Cage::from(cage1)));

    let scream_id2 = id_source.id();
    let screaming2 = summoner.summon(&mut id_source, &scream::from_color(scream_id2, MAGENTA), |_| Outcome::Done);
    let cage2 = Cage::from((-0.4, -0.3, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming2, Wish::FitToCage(Cage::from(cage2)));

    let scream_id3 = id_source.id();
    let screaming3 = summoner.summon(&mut id_source, &scream::from_color(scream_id3, YELLOW), |_| Outcome::Done);
    let cage3 = Cage::from((-0.5, -0.4, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming3, Wish::FitToCage(Cage::from(cage3)));


    let howls = vec![
        howl::create(id_source.id(), BLUE, Cage::from((-0.70, -0.50, -0.10, 0.10, 0.10, 0.10)), Sigil::Fill),
        howl::create(id_source.id(), RED, Cage::from((-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)), Sigil::Fill),
        howl::create(id_source.id(), GREEN, Cage::from((0.25, 0.75, 0.0, 0.5, -0.01, -0.01)), Sigil::Fill),
        howl::create(id_source.id(), CYAN, Cage::from((-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('J')),
        howl::create(id_source.id(), YELLOW, Cage::from((0.00, 0.06, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('y')),
    ];
    for howl in &howls {
        summoner.summon(&mut id_source, howl, |_| Outcome::Done);
    }
    let howl_id = id_source.id();
    summoner.summon(&mut id_source, &howl::misty(howl_id, Default::default()), |_| Outcome::Done);

    Model {
        summoner: summoner,
    }
}

fn update(message: Message, mut model: Model) -> Option<Model> {
    match message {
        Message::Stop => {
            finish(model);
            None
        },
        Message::EmitAnimationFrame => {
            let mut summoner: Summoner = model.summoner.clone();
            summoner.update(Wish::Tick);
            model.summoner = summoner;
            Some(model)
        },
        Message::SetHand(_) => {
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

fn finish(_: Model) {}

pub fn start(viewer: Viewer) -> Sender<Message> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut model = init();
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
                    finish(model);
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

