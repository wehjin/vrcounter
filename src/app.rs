use color::{GREEN, RED, BLUE, CYAN, YELLOW, MAGENTA};
use viewer::{ActiveViewer};
use common::{IdSource};
use scream;
use scream::{ScreamPosition};
use howl;
use howl::{Sigil, Howling};
use scream::{Screaming};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub enum Message {
    Stop,
}

pub fn start(viewer: ActiveViewer) -> Sender<Message> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let model = init(viewer.clone());
        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Stop => {
                    break;
                }
            }
        }
        finish(model);
    });
    tx
}

pub fn stop(agent: Sender<Message>) {
    agent.send(Message::Stop).unwrap_or(())
}

fn init(viewer: ActiveViewer) -> (Vec<Howling>, Vec<Screaming>) {
    let mut id_source = IdSource::new();
    let (message_tx, message_rx) = channel();
    let howls = [
        howl::start::<(), ()>(BLUE, Sigil::Fill, (-0.70, -0.50, -0.10, 0.10, 0.10, 0.10)),
        howl::start::<(), ()>(RED, Sigil::Fill, (-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)),
        howl::start::<(), ()>(GREEN, Sigil::Fill, (0.25, 0.75, 0.0, 0.5, -0.01, -0.01)),
        howl::start::<(), ()>(CYAN, Sigil::Letter('J'), (-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)),
        howl::start::<(), ()>(YELLOW, Sigil::Letter('y'), (0.00, 0.06, -0.03, 0.03, 0.005, 0.005)),
    ];
    let mut howlings = Vec::new();
    for howl in howls.iter() {
        let howling = howl.present(viewer.clone(), message_tx.clone(), &mut id_source);
        howlings.push(howling);
    }

    let position = ScreamPosition { left: -0.5, right: -0.4, top: -0.15, bottom: -0.25, near: 0.03 };
    let scream = scream::of_color(YELLOW)
        .join_right(0.1, scream::of_color(MAGENTA).join_right(0.1, scream::of_color(CYAN)));
    let mut screamings = Vec::new();
    let screaming = scream.present(&position, &mut id_source, viewer.clone());
    screamings.push(screaming);

    (howlings, screamings)
}

fn finish((mut howlings, mut screamings): (Vec<Howling>, Vec<Screaming>)) {
    for howling in &mut howlings {
        howling.silence();
    }
    for screaming in &mut screamings {
        screaming.silence();
    }
}

