extern crate glium;

use glium::{DisplayBuild, Display, Surface};
use glium::glutin::{WindowBuilder};
use cam::Camera;
use programs::Programs;
use keymap::{Keymap, Key};
use app_model::AppModel;
use std::rc::Rc;

pub struct Model {
    display: Rc<Display>,
    programs: Programs,
    keymap: Keymap,
    camera: Camera,
}

impl Model {
    pub fn init(app_model: AppModel) -> Self {
        let display: Rc<Display> = Rc::new(WindowBuilder::new().with_title("vr counter")
                                                               .with_depth_buffer(24)
                                                               .build_glium()
                                                               .unwrap());
        Model {
            display: display.clone(),
            programs: Programs::init(display, app_model.viewer),
            keymap: Keymap::init(),
            camera: Camera::start(),
        }
    }

    pub fn with_camera(self, camera: Camera) -> Self {
        Model {
            display: self.display,
            programs: self.programs,
            keymap: self.keymap,
            camera: camera,
        }
    }
}

pub enum Message {
    Quit,
    Move(Direction),
    Reset,
}

pub fn update(message: &Message, model: Model) -> Option<Model> {
    match *message {
        Message::Quit => None,
        Message::Reset => Some(model.with_camera(Camera::start())),
        Message::Move(ref direction) => {
            let camera = match *direction {
                Direction::Up => model.camera.move_up(),
                Direction::Down => model.camera.move_down(),
                Direction::Left => model.camera.move_left(),
                Direction::Right => model.camera.move_right(),
                Direction::Near => model.camera.move_near(),
                Direction::Far => model.camera.move_far(),
            };
            Some(model.with_camera(camera))
        }
    }
}

pub fn view(model: &Model) -> Message {
    let mut target = model.display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    let (view, perspective) = model.camera.get_view_and_projection(&target);
    model.programs.draw(&mut target, &view, &perspective);
    target.finish().unwrap();
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
    }
    return message_option.unwrap();
}

pub fn run(app_model: AppModel) {
    let mut model = Model::init(app_model);
    loop {
        let message = view(&model);
        match update(&message, model) {
            None => return,
            Some(next_model) => model = next_model,
        }
    }
}

fn message_option_from_key(key: Key) -> Option<Message> {
    match key {
        Key::LookUp => Some(Message::Move(Direction::Up)),
        Key::LookDown => Some(Message::Move(Direction::Down)),
        Key::LookRight => Some(Message::Move(Direction::Right)),
        Key::LookLeft => Some(Message::Move(Direction::Left)),
        Key::LookFar => Some(Message::Move(Direction::Far)),
        Key::LookNear => Some(Message::Move(Direction::Near)),
        Key::ResetLook => Some(Message::Reset),
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
