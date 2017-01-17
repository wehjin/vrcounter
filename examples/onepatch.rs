extern crate vrcounter;
extern crate cage;
extern crate rand;

mod screen_metrics;
mod journal;
mod traveller;
mod caravel;

use vrcounter::*;
use cage::Cage;
use vrcounter::app::{Message as UserEvent};
use screen_metrics::ScreenMetrics;
use journal::Journal2;
use vrcounter::color::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

enum AppMessage {
    Start(ScreenMetrics),
    Step(ScreenMetrics),
    Stop,
}

enum UserMessage {
    AppDidStop
}

pub struct App {
    app_message_writer: std::sync::mpsc::Sender<AppMessage>,
}

use caravel::Caravel;
use std::sync::mpsc::Sender;

impl App {
    fn new<C>(user_message_writer: Sender<UserMessage>, viewer: Viewer, caravel: C)
              -> Self
        where C: Caravel + Send + 'static
    {
        let (app_message_writer, app_message_reader) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut traveller = caravel.embark();
            let mut travel_and_patch = |screen_metrics: ScreenMetrics| {
                let patches_cell = RefCell::new(HashMap::new());
                let journal = Journal2::Prime { screen_metrics: screen_metrics, patches: patches_cell };
                let rc_journal = Rc::new(journal);
                traveller.travel(rc_journal.clone());
                viewer.clear();
                let patches = rc_journal.patches();
                for (_, patch) in patches {
                    viewer.add_patch(patch);
                }
            };
            loop {
                match app_message_reader.recv().unwrap() {
                    AppMessage::Start(screen_metrics) => travel_and_patch(screen_metrics),
                    AppMessage::Step(screen_metrics) => travel_and_patch(screen_metrics),
                    AppMessage::Stop => {
                        user_message_writer.send(UserMessage::AppDidStop).unwrap();
                        break;
                    }
                }
            }
        });
        App { app_message_writer: app_message_writer }
    }
    fn send(&self, app_message: AppMessage) {
        self.app_message_writer.send(app_message).unwrap();
    }
}

fn main() {
    use caravel::color::ColorCaravel;
    use caravel::spectrum::SpectrumCaravel;

    let viewer = Viewer::start();
    let cage = Cage::from((-0.5, 0.5, -1.5, 0.0, 0.0, 0.2));
    let screen_metrics = ScreenMetrics::new(cage, 0.03, 0.01);
    let glyffiary = glyffin::Glyffiary::new();
    let sigil = Sigil::of_line("Jupiter", &glyffiary);

    let top_caravel = ColorCaravel::new(YELLOW, Sigil::of_fill())
        .dock_left(24.0, ColorCaravel::new(AZURE, sigil))
        .dock_left(1.0, SpectrumCaravel::new());

    let caravel = ColorCaravel::new(VIOLET, Sigil::of_fill())
        .dock_top(3.0, top_caravel);

    let (main_message_writer, main_message_reader) = std::sync::mpsc::channel();
    let app = App::new(main_message_writer, viewer.clone(), caravel);
    app.send(AppMessage::Start(screen_metrics));

    gl_user::run(viewer.clone(), |x: UserEvent| match x {
        UserEvent::EmitAnimationFrame => app.send(AppMessage::Step(screen_metrics)),
        UserEvent::Stop => println!("UserEvent::Stop"),
        _ => ()
    });

    app.send(AppMessage::Stop);
    loop {
        match main_message_reader.recv().unwrap() {
            UserMessage::AppDidStop => {
                break
            }
        }
    }
    viewer.stop();
}
