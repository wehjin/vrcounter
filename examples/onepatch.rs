extern crate vrcounter;
extern crate cage;
extern crate rand;

mod screen_metrics;
mod journal;
mod traveller;

use vrcounter::*;
use cage::Cage;
use vrcounter::app::{Message as UserEvent};
use screen_metrics::ScreenMetrics;
use journal::{PrimeJournal};
use traveller::{Traveller, PatchTraveller};

enum AppMessage {
    Go(ScreenMetrics),
    Stop,
}

enum UserMessage {
    AppDidStop
}

pub struct App {
    app_message_writer: std::sync::mpsc::Sender<AppMessage>,
}

impl App {
    fn new(user_message_writer: std::sync::mpsc::Sender<UserMessage>, viewer: Viewer) -> Self {
        let (app_message_writer, app_message_reader) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            loop {
                match app_message_reader.recv().unwrap() {
                    AppMessage::Go(screen_metrics) => {
                        let mut journal = PrimeJournal::new(screen_metrics);
                        let mut traveller = PatchTraveller::new();
                        traveller.travel(&mut journal);

                        viewer.clear();
                        for (_, patch) in journal.patches() {
                            viewer.add_patch(*patch);
                        }
                    }
                    AppMessage::Stop => {
                        user_message_writer.send(UserMessage::AppDidStop).unwrap();
                        return;
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
    let viewer = Viewer::start();
    let cage = Cage::from((-0.5, 0.5, -1.5, 0.0, 0.0, 0.2));
    let screen_metrics = ScreenMetrics::new(cage, 0.03, 0.01);

    let (user_message_writer, user_message_reader) = std::sync::mpsc::channel();
    let app = App::new(user_message_writer.clone(), viewer.clone());
    app.send(AppMessage::Go(screen_metrics));

    let (user_event_writer, user_event_reader) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        loop {
            match user_event_reader.recv().unwrap() {
                UserEvent::Stop => {
                    println!("UserEvent::Stop");
                    app.send(AppMessage::Stop);
                    return;
                },
                _ => ()
            }
        }
    });
    gl_user::run(viewer.clone(), user_event_writer.clone());
    user_event_writer.send(UserEvent::Stop).unwrap();

    loop {
        match user_message_reader.recv().unwrap() {
            UserMessage::AppDidStop => {
                break
            }
        }
    }
    viewer.stop();
}
