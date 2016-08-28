extern crate glium;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState};
use world;
use cam;
use std::env;

pub struct Model {
    display: glium::Display,
    room: world::PatchProgram,
    camera: cam::Camera,
    is_windows: bool,
}

impl Model {
    pub fn init() -> Self {
        let display: glium::Display = glium::glutin::WindowBuilder::new()
            .with_title("vrcounter")
            .with_depth_buffer(24)
            .build_glium()
            .unwrap();
        let room = world::PatchProgram::new(&display);
        let camera = cam::Camera::start();
        let is_windows = match env::var("HOME") {
            Ok(val) => {
                if val.starts_with("/Users/") {
                    false
                } else {
                    true
                }
            },
            Err(_) => true
        };
        Model { display: display, room: room, camera: camera, is_windows: is_windows }
    }

    pub fn with_camera(self, camera: cam::Camera) -> Self {
        Model { display: self.display, room: self.room, camera: camera, is_windows: self.is_windows }
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
    model.room.draw_to_camera(&mut target, &model.camera);
    target.finish().unwrap();
    let mut message_option: Option<Message> = None;
    while message_option.is_none() {
        for glutin_event in model.display.poll_events() {
            if model.is_windows {
                message_option = match glutin_event {
                    Event::Closed => Some(Message::Quit),
                    Event::KeyboardInput(ElementState::Pressed, 1, _) => Some(Message::Quit),
                    Event::KeyboardInput(ElementState::Pressed, 52, _) => Some(Message::Reset),
                    Event::KeyboardInput(ElementState::Pressed, 30, _) => Some(Message::Move(Direction::Left)),
                    Event::KeyboardInput(ElementState::Pressed, 32, _) => Some(Message::Move(Direction::Right)),
                    Event::KeyboardInput(ElementState::Pressed, 17, _) => Some(Message::Move(Direction::Up)),
                    Event::KeyboardInput(ElementState::Pressed, 31, _) => Some(Message::Move(Direction::Down)),
                    Event::KeyboardInput(ElementState::Pressed, 16, _) => Some(Message::Move(Direction::Far)),
                    Event::KeyboardInput(ElementState::Pressed, 18, _) => Some(Message::Move(Direction::Near)),
                    Event::KeyboardInput(ElementState::Pressed, code, _) => {
                        println!("{}", code);
                        None
                    },
                    _ => None
                };
            } else {
                message_option = match glutin_event {
                    Event::Closed => Some(Message::Quit),
                    Event::KeyboardInput(ElementState::Pressed, 53, _) => Some(Message::Quit),
                    Event::KeyboardInput(ElementState::Pressed, 47, _) => Some(Message::Reset),
                    Event::KeyboardInput(ElementState::Pressed, 1, _) => Some(Message::Move(Direction::Down)),
                    Event::KeyboardInput(ElementState::Pressed, 13, _) => Some(Message::Move(Direction::Up)),
                    Event::KeyboardInput(ElementState::Pressed, 0, _) => Some(Message::Move(Direction::Left)),
                    Event::KeyboardInput(ElementState::Pressed, 2, _) => Some(Message::Move(Direction::Right)),
                    Event::KeyboardInput(ElementState::Pressed, 12, _) => Some(Message::Move(Direction::Far)),
                    Event::KeyboardInput(ElementState::Pressed, 14, _) => Some(Message::Move(Direction::Near)),
                    Event::KeyboardInput(ElementState::Pressed, code, _) => {
                        println!("{}", code);
                        None
                    },
                    _ => None
                };
            }
            if message_option.is_some() {
                break;
            }
        }
    }
    return message_option.unwrap();
}
