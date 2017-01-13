extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::*;
use vrcounter::color::*;
use cage::Cage;

use vrcounter::app::{Message as UserEvent};

enum AppMessage {
    Go,
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
                    AppMessage::Go => {
                        let color = SPECTRUM[0 % SPECTRUM.len()];
                        let cage = Cage::from((-0.5, 0.5, -1.5, 0.0, 0.0, 0.2));
                        let patch_id = rand::random::<u64>();
                        viewer.add_patch(Patch::from_cage(&cage, color, Sigil::Fill, patch_id));
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

    let (user_message_writer, user_message_reader) = std::sync::mpsc::channel();
    let app = App::new(user_message_writer.clone(), viewer.clone());
    app.send(AppMessage::Go);

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
