extern crate cage;

use color::{GREEN, RED, BLUE, CYAN, YELLOW, MAGENTA};
use viewer::{ActiveViewer};
use common::{IdSource};
use scream;
use scream::{ScreamPosition};
use scream::{Screaming};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;
use summoner::{Summoner, DemonVision};
use roar;
use color;
use std::boxed::Box;
use cage::Cage;
use patch::Sigil;
use vision;

pub enum Message {
    Stop,
    EmitAnimationFrame,
}

struct Model {
    screamings: Vec<Screaming>,
    summoner: Summoner,
}

pub enum Outcome {
    Done,
}


fn init(viewer: ActiveViewer) -> Model {
    use howl;
    let mut id_source = IdSource::new();

    let position = ScreamPosition { left: -0.5, right: -0.4, top: -0.15, bottom: -0.25, near: 0.03 };
    let scream = scream::of_color(YELLOW)
        .join_right(0.1, scream::of_color(MAGENTA).join_right(0.1, scream::of_color(CYAN)));
    let mut screamings = Vec::new();
    let screaming = scream.present(&position, &mut id_source, viewer.clone());
    screamings.push(screaming);

    let mut summoner = Summoner::new();
    let roar = roar::demo::from(vec![color::GREEN, color::RED, color::BLUE, color::CYAN, color::MAGENTA, color::YELLOW]);
    summoner.summon(&mut id_source, &roar, |_| Outcome::Done);

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
        screamings: screamings,
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
            summoner.update(vision::Outcome::Tick);
            model.summoner = summoner;
            Some(model)
        },
    }
}

fn view(model: &Model, viewer: &ActiveViewer) {
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

fn finish(mut model: Model) {
    // TODO: Silence roaring and howlings?
    for screaming in &mut model.screamings {
        screaming.silence();
    }
}

pub fn start(viewer: ActiveViewer) -> Sender<Message> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let mut model = init(viewer.clone());
        view(&model, &viewer);
        loop {
            match rx.recv() {
                Ok(message) => {
                    match update(message, model) {
                        Option::None => {
                            break;
                        },
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

