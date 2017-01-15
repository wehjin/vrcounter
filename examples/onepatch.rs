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
use journal::{PrimeJournal};
use traveller::{Traveller};
use vrcounter::color::*;

enum AppMessage {
    Go(ScreenMetrics),
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

impl App {
    fn new<C, T>(
        user_message_writer: std::sync::mpsc::Sender<UserMessage>,
        viewer: Viewer,
        caravel: C
    ) -> Self
        where T: Traveller, C: Caravel<T> + Send + 'static
    {
        let (app_message_writer, app_message_reader) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut traveller = caravel.embark();
            let mut travel_and_patch = |screen_metrics: ScreenMetrics| {
                let mut journal = PrimeJournal::new(screen_metrics);
                traveller.travel(&mut journal);
                viewer.clear();
                for (_, patch) in journal.patches() {
                    viewer.add_patch(*patch);
                }
            };
            loop {
                match app_message_reader.recv().unwrap() {
                    AppMessage::Go(screen_metrics) => travel_and_patch(screen_metrics),
                    AppMessage::Step(screen_metrics) => travel_and_patch(screen_metrics),
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
    use caravel::color::ColorCaravel;
    use caravel::spectrum::SpectrumCaravel;

    let viewer = Viewer::start();
    let cage = Cage::from((-0.5, 0.5, -1.5, 0.0, 0.0, 0.2));
    let screen_metrics = ScreenMetrics::new(cage, 0.03, 0.01);

    let glyfficon = glyffin::Glyffiary::new();

    let top_caravel = ColorCaravel::new(YELLOW, Sigil::Fill)
        .dock_left(24.0, ColorCaravel::new(AZURE, Sigil::FitLetter('J', glyfficon)))
        .dock_left(1.0, SpectrumCaravel::new());

    let caravel = ColorCaravel::new(VIOLET, Sigil::Fill)
        .dock_top(3.0, top_caravel);

    let (user_message_writer, user_message_reader) = std::sync::mpsc::channel();
    let app = App::new(user_message_writer.clone(), viewer.clone(), caravel);
    app.send(AppMessage::Go(screen_metrics));

    let (user_event_writer, user_event_reader) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        loop {
            match user_event_reader.recv().unwrap() {
                UserEvent::EmitAnimationFrame => {
                    app.send(AppMessage::Step(screen_metrics));
                },
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
