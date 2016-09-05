use color::{GREEN, RED, BLUE, CYAN, YELLOW, MAGENTA};
use viewer::{ActiveViewer};
use common::{IdSource};
use scream;
use scream::{ScreamPosition};
use howl;
use howl::{Sigil, Howling, Message as HowlMessage};
use scream::{Screaming};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::collections::HashMap;
use summoner::{Summoner, Demon, DemonVision};
use roar;
use color;
use std::boxed::Box;
use patch::Patch;

pub enum Message {
    Stop,
}

struct Model {
    howlings: Vec<Howling>,
    screamings: Vec<Screaming>,
    howl_message_receiver: Receiver<HowlMessage<(), ()>>,
    summoner: Summoner,
    roaring: u64,
}

pub enum Outcome {
    Done,
}

fn view(model: &Model, viewer: &ActiveViewer) {
    let demon_boxes = model.summoner.get_demon_boxes();
    for demon_box in demon_boxes {
        let demon_vision_box: Box<DemonVision> = (&demon_box).see();
        let demon_patches: &HashMap<u64, Patch> = (*demon_vision_box).patches();
        for (id, patch) in demon_patches.iter() {
            viewer.add_patch(*patch);
        }
    }
}

pub fn start(viewer: ActiveViewer) -> Sender<Message> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let model = init(viewer.clone());
        view(&model, &viewer);
        loop {
            match rx.recv() {
                Ok(msg) => match msg {
                    Message::Stop => { break; },
                },
                Err(err) => {
                    println!("ERROR: {:?}", err);
                    break;
                },
            };
        }
        finish(model);
    });
    tx
}

pub fn stop(agent: Sender<Message>) {
    agent.send(Message::Stop).unwrap_or(())
}

fn init(viewer: ActiveViewer) -> Model {
    let mut id_source = IdSource::new();
    let (howl_message_sender, howl_message_receiver) = channel();
    let howls = [
        howl::start::<(), ()>(BLUE, Sigil::Fill, (-0.70, -0.50, -0.10, 0.10, 0.10, 0.10)),
        howl::start::<(), ()>(RED, Sigil::Fill, (-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)),
        howl::start::<(), ()>(GREEN, Sigil::Fill, (0.25, 0.75, 0.0, 0.5, -0.01, -0.01)),
        howl::start::<(), ()>(CYAN, Sigil::Letter('J'), (-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)),
        howl::start::<(), ()>(YELLOW, Sigil::Letter('y'), (0.00, 0.06, -0.03, 0.03, 0.005, 0.005)),
    ];
    let mut howlings = Vec::new();
    for howl in howls.iter() {
        let howling = howl.present(viewer.clone(), howl_message_sender.clone(), &mut id_source);
        howlings.push(howling);
    }

    let position = ScreamPosition { left: -0.5, right: -0.4, top: -0.15, bottom: -0.25, near: 0.03 };
    let scream = scream::of_color(YELLOW)
        .join_right(0.1, scream::of_color(MAGENTA).join_right(0.1, scream::of_color(CYAN)));
    let mut screamings = Vec::new();
    let screaming = scream.present(&position, &mut id_source, viewer.clone());
    screamings.push(screaming);

    let mut summoner = Summoner::new();
    let roar = roar::color::from(vec![color::GREEN, color::RED, color::BLUE, color::CYAN, color::MAGENTA, color::YELLOW]);
    let roaring = summoner.summon(&mut id_source, &roar, Box::new(move |outcome: roar::color::Outcome| -> Outcome {
        Outcome::Done
    }));

    Model {
        howlings: howlings,
        screamings: screamings,
        howl_message_receiver: howl_message_receiver,
        summoner: summoner,
        roaring: roaring,
    }
}

fn finish(mut model: Model) {
    for howling in &mut model.howlings {
        howling.silence();
    }
    for screaming in &mut model.screamings {
        screaming.silence();
    }
}

