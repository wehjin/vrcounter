extern crate glium;

use glium::{DisplayBuild, Display, Surface};
use glium::glutin::{WindowBuilder};
use cam::Camera;
use programs::Programs;
use keymap::{Keymap, Key};
use std::rc::Rc;
use viewer::Viewer;
use std::time::{Instant, Duration};
use hand::Hand;
use user::UserEvent;
use std::boxed::Box;
use sakura::PressLabel;
use sakura::AsciiPoint;
use glium::glutin::{Event, ElementState, VirtualKeyCode};


pub struct Model {
    display: Rc<Display>,
    programs: Programs,
    keymap: Keymap,
    camera: Camera,
    on_event: Box<Fn(UserEvent) -> ()>,
    hand: Hand,
    viewer: Viewer,
}

pub enum Message {
    Quit,
    MoveCamera(Direction),
    MoveHand(Direction),
    ResetCamera,
    EmitAnimationFrame,
    Press(PressLabel),
    Release(PressLabel),
}

pub fn run<F>(viewer: Viewer, on_event: F) where F: Fn(UserEvent) -> () + 'static
{
    let mut model = init(viewer, on_event);
    loop {
        let message = draw(&model);
        match update(message, model) {
            None => return,
            Some(next_model) => model = next_model,
        }
    }
}

pub fn init<F>(viewer: Viewer, on_event: F) -> Model where F: Fn(UserEvent) -> () + 'static
{
    use programs::HandType;
    let display: Rc<Display> = Rc::new(WindowBuilder::new().with_title("vr counter")
                                                           .with_depth_buffer(24)
                                                           .build_glium()
                                                           .unwrap());
    Model {
        on_event: Box::new(on_event),
        display: display.clone(),
        programs: Programs::new(display, viewer.clone(), HandType::Keyboard),
        keymap: Keymap::init(),
        camera: Camera::start(),
        hand: Default::default(),
        viewer: viewer.clone(),
    }
}

pub fn update(message: Message, mut model: Model) -> Option<Model>
{
    match message {
        Message::Quit => None,
        Message::ResetCamera => Some(model.with_camera(Camera::start())),
        Message::MoveCamera(direction) => {
            let camera = get_camera(&model, direction);
            Some(model.with_camera(camera))
        },
        Message::EmitAnimationFrame => {
            (model.on_event)(UserEvent::EmitAnimationFrame);
            Some(model)
        },
        Message::MoveHand(direction) => {
            const STEP: f32 = 0.05;
            let (dx, dy, dz) = match direction {
                Direction::Up => (0.0, STEP, 0.0),
                Direction::Down => (0.0, -STEP, 0.0),
                Direction::Left => (-STEP, 0.0, 0.0),
                Direction::Right => (STEP, 0.0, 0.0),
                Direction::Far => (0.0, 0.0, -STEP),
                Direction::Near => (0.0, 0.0, STEP),
            };
            let offset = model.hand.offset.shift(dx, dy, dz);
            model.hand.offset = offset;
            model.viewer.set_hand(model.hand);
            (model.on_event)(UserEvent::SetHand(model.hand));
            Some(model)
        },
        Message::Press(label) => {
            (model.on_event)(UserEvent::Press(label));
            Some(model)
        },
        Message::Release(label) => {
            (model.on_event)(UserEvent::Release(label));
            Some(model)
        },
    }
}

fn get_camera(model: &Model, direction: Direction) -> Camera
{
    match direction {
        Direction::Up => model.camera.move_up(),
        Direction::Down => model.camera.move_down(),
        Direction::Left => model.camera.move_left(),
        Direction::Right => model.camera.move_right(),
        Direction::Near => model.camera.move_near(),
        Direction::Far => model.camera.move_far(),
    }
}

impl PressLabel {
    fn to_message(self, elementstate: ElementState) -> Option<Message> {
        Some(match elementstate {
            ElementState::Pressed => Message::Press(self),
            ElementState::Released => Message::Release(self),
        })
    }
}

pub fn draw(model: &Model) -> Message
{
    let mut target = model.display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    let (view, perspective) = model.camera.get_view_and_projection(&target);
    model.programs.draw(&mut target, &view, &perspective);
    target.finish().unwrap();

    let frame_instant = Instant::now();
    let frame_duration = Duration::from_millis(300);

    let mut message_option: Option<Message> = None;
    'find_message: while message_option.is_none() {
        for glutin_event in model.display.poll_events() {
            message_option = match glutin_event {
                Event::KeyboardInput(elementstate, _, Some(VirtualKeyCode::Y)) => PressLabel::Ascii(AsciiPoint::Y).to_message(elementstate),
                Event::KeyboardInput(elementstate, _, Some(VirtualKeyCode::U)) => PressLabel::Ascii(AsciiPoint::U).to_message(elementstate),
                Event::KeyboardInput(elementstate, _, Some(VirtualKeyCode::Left)) => PressLabel::SelectionEditLeft.to_message(elementstate),
                Event::KeyboardInput(elementstate, _, Some(VirtualKeyCode::Back)) => PressLabel::Ascii(AsciiPoint::Backspace).to_message(elementstate),
                _ => None,
            };
            if message_option.is_some() {
                break 'find_message;
            }

            if let Some(key) = model.keymap.key_for_event(&glutin_event) {
                message_option = message_option_from_key(key);
                if message_option.is_some() {
                    break 'find_message;
                }
            }
        }
        if Instant::now().duration_since(frame_instant) > frame_duration {
            message_option = Some(Message::EmitAnimationFrame);
        }
    }
    return message_option.unwrap();
}

impl Model
{
    pub fn with_camera(self, camera: Camera) -> Self {
        Model {
            display: self.display,
            programs: self.programs,
            keymap: self.keymap,
            camera: camera,
            on_event: self.on_event,
            hand: self.hand,
            viewer: self.viewer,
        }
    }
}

fn message_option_from_key(key: Key) -> Option<Message> {
    match key {
        Key::LookUp => Some(Message::MoveCamera(Direction::Up)),
        Key::LookDown => Some(Message::MoveCamera(Direction::Down)),
        Key::LookRight => Some(Message::MoveCamera(Direction::Right)),
        Key::LookLeft => Some(Message::MoveCamera(Direction::Left)),
        Key::StepBack => Some(Message::MoveCamera(Direction::Far)),
        Key::StepForward => Some(Message::MoveCamera(Direction::Near)),
        Key::ResetLook => Some(Message::ResetCamera),
        Key::Quit => Some(Message::Quit),
        Key::HandLeft => Some(Message::MoveHand(Direction::Left)),
        Key::HandDown => Some(Message::MoveHand(Direction::Down)),
        Key::HandUp => Some(Message::MoveHand(Direction::Up)),
        Key::HandRight => Some(Message::MoveHand(Direction::Right)),
        Key::HandForward => Some(Message::MoveHand(Direction::Far)),
        Key::HandBack => Some(Message::MoveHand(Direction::Near)),
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
