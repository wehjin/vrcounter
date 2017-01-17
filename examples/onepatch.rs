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
use journal::Journal;
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
    fn new<C>(user_message_writer: Sender<UserMessage>, viewer: Viewer, caravel: C) -> Self
        where C: Caravel + Send + 'static
    {
        let (app_message_writer, app_message_reader) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut traveller = caravel.embark();
            let mut travel_and_patch = |screen_metrics: ScreenMetrics| {
                let shared_journal = Rc::new(Journal::Prime {
                    screen_metrics: screen_metrics,
                    patches: RefCell::new(HashMap::new())
                });

                traveller.travel(shared_journal.clone());
                viewer.set_patches(shared_journal.patches());
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
    use caravel::Caravel;
    use caravel::color::ColorCaravel;
    use caravel::spectrum::SpectrumCaravel;

    let viewer = Viewer::start();
    let cage = Cage::from((-0.5, 0.5, -1.5, 0.0, 0.0, 0.2));
    let screen_metrics = ScreenMetrics::new(cage, 0.03, 0.01);
    let glyffiary = glyffin::Glyffiary::new();

    let top_caravel = ColorCaravel::new(Sigil::of_line("Jupiter", &glyffiary), AZURE)
        .contract(1.0, 0.5)
        .dock_left(1.0, SpectrumCaravel::new());

    let caravel = ColorCaravel::new(Sigil::of_fill(), VIOLET)
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
