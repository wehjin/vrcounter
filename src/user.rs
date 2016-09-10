extern crate glium;

use glium::{DisplayBuild, Display, Surface};
use glium::glutin::{WindowBuilder};
use cam::Camera;
use programs::Programs;
use keymap::{Keymap, Key};
use std::rc::Rc;
use viewer::ActiveViewer;
use app::{Message as AppMessage};
use std::sync::mpsc::Sender;
use std::time::{Instant, Duration};

pub struct Model {
    display: Rc<Display>,
    programs: Programs,
    keymap: Keymap,
    camera: Camera,
    app: Sender<AppMessage>,
}

pub enum Message {
    Quit,
    MoveCamera(Direction),
    ResetCamera,
    EmitAnimationFrame,
}

pub fn run(viewer: ActiveViewer, app: Sender<AppMessage>) {
    let mut model = init(viewer, app);
    loop {
        let message = draw(&model);
        match update(&message, model) {
            None => return,
            Some(next_model) => model = next_model,
        }
    }
}

pub fn init(viewer: ActiveViewer, app: Sender<AppMessage>) -> Model {
    let display: Rc<Display> = Rc::new(WindowBuilder::new().with_title("vr counter")
                                                           .with_depth_buffer(24)
                                                           .build_glium()
                                                           .unwrap());
    Model {
        app: app,
        display: display.clone(),
        programs: Programs::init(display, viewer, false),
        keymap: Keymap::init(),
        camera: Camera::start(),
    }
}

pub fn update(message: &Message, model: Model) -> Option<Model> {
    match *message {
        Message::Quit => None,
        Message::ResetCamera => Some(model.with_camera(Camera::start())),
        Message::MoveCamera(ref direction) => {
            let camera = match *direction {
                Direction::Up => model.camera.move_up(),
                Direction::Down => model.camera.move_down(),
                Direction::Left => model.camera.move_left(),
                Direction::Right => model.camera.move_right(),
                Direction::Near => model.camera.move_near(),
                Direction::Far => model.camera.move_far(),
            };
            Some(model.with_camera(camera))
        },
        Message::EmitAnimationFrame => {
            model.app.send(AppMessage::EmitAnimationFrame).unwrap_or(());
            Some(model)
        }
    }
}

pub fn draw(model: &Model) -> Message {
    let mut target = model.display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    let (view, perspective) = model.camera.get_view_and_projection(&target);
    model.programs.draw(&mut target, &view, &perspective);
    target.finish().unwrap();

    let frame_instant = Instant::now();
    let frame_duration = Duration::from_millis(300);

    let mut message_option: Option<Message> = None;
    while message_option.is_none() {
        for glutin_event in model.display.poll_events() {
            if let Some(key) = model.keymap.key_for_event(&glutin_event) {
                message_option = message_option_from_key(key);
                if message_option.is_some() {
                    break;
                }
            }
        }
        if Instant::now().duration_since(frame_instant) > frame_duration {
            message_option = Some(Message::EmitAnimationFrame);
        }
    }
    return message_option.unwrap();
}

impl Model {
    pub fn with_camera(self, camera: Camera) -> Self {
        Model {
            display: self.display,
            programs: self.programs,
            keymap: self.keymap,
            camera: camera,
            app: self.app,
        }
    }
}

fn message_option_from_key(key: Key) -> Option<Message> {
    match key {
        Key::LookUp => Some(Message::MoveCamera(Direction::Up)),
        Key::LookDown => Some(Message::MoveCamera(Direction::Down)),
        Key::LookRight => Some(Message::MoveCamera(Direction::Right)),
        Key::LookLeft => Some(Message::MoveCamera(Direction::Left)),
        Key::LookFar => Some(Message::MoveCamera(Direction::Far)),
        Key::LookNear => Some(Message::MoveCamera(Direction::Near)),
        Key::ResetLook => Some(Message::ResetCamera),
        Key::Quit => Some(Message::Quit),
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Near,
    Far
}
