extern crate glium;

use glium::{DisplayBuild, Surface, Display};
use glium::glutin::{Event, ElementState, WindowBuilder};
use patch_program::{PatchProgram};
use cam;
use os;
use mat;
use shape::ShapeList;
use std::f32::consts::PI;
use programs::Programs;

pub struct Model {
    display: Display,
    programs: Programs,
    patch_program: PatchProgram,
    camera: cam::Camera,
    is_windows: bool,
}

impl Model {
    pub fn init(shape_list: ShapeList) -> Self {
        let display: Display = WindowBuilder::new().with_title("vr counter")
                                                   .with_depth_buffer(24)
                                                   .build_glium()
                                                   .unwrap();
        Model {
            programs: Programs::new(&display),
            patch_program: PatchProgram::new(&display, shape_list),
            camera: cam::Camera::start(),
            is_windows: os::is_windows(),
            display: display,
        }
    }

    pub fn with_camera(self, camera: cam::Camera) -> Self {
        Model {
            display: self.display,
            programs: self.programs,
            patch_program: self.patch_program,
            camera: camera,
            is_windows: self.is_windows,
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
    model.patch_program.draw(&mut target, &view, &perspective);
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
