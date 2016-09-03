extern crate glium;

use glium::{DisplayBuild, Display, Surface};
use glium::glutin::{WindowBuilder};
use cam;
use mat;
use std::f32::consts::PI;
use programs::Programs;
use shape::ShapeList;
use keymap::{Keymap, Key};

pub struct Model {
    display: Display,
    programs: Programs,
    camera: cam::Camera,
    keymap: Keymap,
}

impl Model {
    pub fn init(shape_list: ShapeList) -> Self {
        let display: Display = WindowBuilder::new().with_title("vr counter")
                                                   .with_depth_buffer(24)
                                                   .build_glium()
                                                   .unwrap();
        Model {
            programs: Programs::init(&display, shape_list),
            camera: cam::Camera::start(),
            display: display,
            keymap: Keymap::init(),
        }
    }

    pub fn with_camera(self, camera: cam::Camera) -> Self {
        Model {
            display: self.display,
            programs: self.programs,
            camera: camera,
            keymap: Keymap::init(),
        }
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

pub enum Message {
    Quit,
    Move(Direction),
    Reset,
}

pub fn update(message: &Message, model: Model) -> Option<Model> {
    match *message {
        Message::Quit => None,
        Message::Reset => Some(model.with_camera(cam::Camera::start())),
        Message::Move(ref direction) => {
            let camera = match *direction {
                Direction::Up => model.camera.up(),
                Direction::Down => model.camera.down(),
                Direction::Left => model.camera.left(),
                Direction::Right => model.camera.right(),
                Direction::Near => model.camera.near(),
                Direction::Far => model.camera.far(),
            };
            Some(model.with_camera(camera))
        }
    }
}

pub fn view(model: &Model) -> Message {
    let mut target = model.display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let camera = &model.camera;
    let view = mat::view_matrix(&camera.eye, &camera.look, &camera.up);
    let perspective = mat::perspective_matrix(target.get_dimensions(), PI / 3.0);
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
