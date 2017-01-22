extern crate vrcounter;
extern crate cage;
extern crate rand;

mod screen_metrics;
mod journal;
mod traveller;
mod caravel;

use vrcounter::*;
use cage::Cage;
use screen_metrics::ScreenMetrics;
use journal::Journal;
use vrcounter::color::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use vrcounter::glyffin::Glyffiary;
use traveller::Traveller;
use vrcounter::user::UserEvent;
use vrcounter::sakura::{PressLabel, Pressboard};
use caravel::Caravel;
use std::sync::mpsc::Sender;

enum AppMessage {
    Frame,
    Press(PressLabel),
    Release(PressLabel),
    Start,
    Stop,
}

enum UserMessage {
    AppDidStop
}

#[derive(Clone, Debug)]
pub struct App {
    app_message_writer: std::sync::mpsc::Sender<AppMessage>,
}

impl App {
    fn new<C>(user_message_writer: Sender<UserMessage>, viewer: Viewer, caravel: C,
              glyffiary: Glyffiary, screen_metrics: ScreenMetrics)
              -> Self
        where C: Caravel + Send + 'static
    {
        let (app_message_writer, app_message_reader) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let shared_glyffiary = Rc::new(glyffiary);
            let mut traveller = caravel.embark();
            let mut app_time: u64 = 1;
            let shared_pressboard = Rc::new(RefCell::new(Pressboard::new()));
            loop {
                match app_message_reader.recv().unwrap() {
                    AppMessage::Start => {
                        App::update(&mut traveller,
                                    screen_metrics, &viewer, shared_glyffiary.clone(), shared_pressboard.clone(), app_time)
                    },
                    AppMessage::Frame => {
                        App::update(&mut traveller,
                                    screen_metrics, &viewer, shared_glyffiary.clone(), shared_pressboard.clone(), app_time)
                    },
                    AppMessage::Press(label) => {
                        { shared_pressboard.borrow_mut().begin_press(label, app_time); }
                        App::update(&mut traveller,
                                    screen_metrics, &viewer, shared_glyffiary.clone(), shared_pressboard.clone(), app_time);
                    },
                    AppMessage::Release(label) => {
                        { shared_pressboard.borrow_mut().end_press(label, app_time); }
                        App::update(&mut traveller,
                                    screen_metrics, &viewer, shared_glyffiary.clone(), shared_pressboard.clone(), app_time);
                    },
                    AppMessage::Stop => {
                        user_message_writer.send(UserMessage::AppDidStop).unwrap();
                        break;
                    }
                }
                app_time = app_time + 1;
            }
        });
        App { app_message_writer: app_message_writer }
    }
    fn send(&self, app_message: AppMessage) {
        self.app_message_writer.send(app_message).unwrap();
    }
    fn update(traveller: &mut Traveller,
              screen_metrics: ScreenMetrics, viewer: &Viewer, shared_glyffiary: Rc<Glyffiary>,
              board: Rc<RefCell<Pressboard>>, today: u64)
    {
        let shared_journal = Rc::new(Journal::Prime {
            screen_metrics: screen_metrics,
            patches: RefCell::new(HashMap::new()),
            shared_glyffiary: shared_glyffiary,
            shared_pressboard: board,
            time: today,
        });
        traveller.travel(shared_journal.clone());
        viewer.set_patches(shared_journal.patches());
    }
}

fn main() {
    use caravel::Caravel;
    use caravel::color::ColorCaravel;

    let viewer = Viewer::start();
    let cage = Cage::from((-0.5, 0.5, -1.5, 0.0, 0.0, 0.2));
    let screen_metrics = ScreenMetrics::new(cage, 0.03, 0.01);
    let glyffiary = glyffin::Glyffiary::new();

    let line_editor = caravel::new_line_editor("Saturn", 7, 'a', AZURE);
    let top_caravel = line_editor.contract(1.0, 0.5);

    let caravel = ColorCaravel::new(Sigil::of_fill(), VIOLET)
        .dock_top(3.0, top_caravel);

    let (main_message_writer, main_message_reader) = std::sync::mpsc::channel();
    let app = App::new(main_message_writer, viewer.clone(), caravel, glyffiary, screen_metrics);
    app.send(AppMessage::Start);

    let gl_app = app.clone();
    gl_user::run(viewer.clone(), move |x: UserEvent| match x {
        UserEvent::EmitAnimationFrame => gl_app.send(AppMessage::Frame),
        UserEvent::Press(label) => gl_app.send(AppMessage::Press(label)),
        UserEvent::Release(label) => gl_app.send(AppMessage::Release(label)),
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
