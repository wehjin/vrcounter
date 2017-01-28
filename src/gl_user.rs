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
    preview: Option<f32>
}

impl Model
{
    pub fn with_camera(self, camera: Camera) -> Self {
        Model { camera: camera, ..self }
    }
    pub fn with_preview(self, preview: Option<f32>) -> Self {
        Model { preview: preview, ..self }
    }
}

pub enum Message {
    Quit,
    MoveCamera(Direction),
    MoveHand(Direction),
    ResetCamera,
    EmitAnimationFrame,
    Press(PressLabel),
    Release(PressLabel),
    StartWheel,
    MoveWheel(f32),
    EndWheel,
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
        preview: None,
    }
}

pub fn update(message: Message, mut model: Model) -> Option<Model>
{
    match message {
        Message::StartWheel => {
            println!("StartWheel");
            Some(model)
        },
        Message::MoveWheel(distance) => {
            let (is_new_previous_distance, previous_distance) = match model.preview {
                Some(distance) => (false, distance),
                None => (true, 0.0),
            };
            let new_distance: f32 = previous_distance + distance / 15.0;
            let previous_ascii_point = AsciiPoint::from(previous_distance - 78.0);
            let new_ascii_point = AsciiPoint::from(new_distance - 78.0);
            if new_ascii_point != previous_ascii_point || is_new_previous_distance {
                println!("Wheel {:?}", new_ascii_point);
                (model.on_event)(UserEvent::Preview(Some(new_ascii_point)));
            }
            Some(model.with_preview(Some(new_distance)))
        },
        Message::EndWheel => {
            println!("EndWheel");
            (model.on_event)(UserEvent::Preview(None));
            Some(model.with_preview(None))
        },
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
            use glium::glutin::MouseScrollDelta;
            use glium::glutin::TouchPhase;

            message_option = match glutin_event {
                Event::KeyboardInput(elementstate, _, Some(VirtualKeyCode::Left)) => PressLabel::SelectionEditLeft.to_message(elementstate),
                Event::KeyboardInput(elementstate, _, Some(VirtualKeyCode::Back)) => PressLabel::Ascii(AsciiPoint::Backspace).to_message(elementstate),
                Event::KeyboardInput(elementstate, _, Some(VirtualKeyCode::Space)) => PressLabel::Ascii(AsciiPoint::Space).to_message(elementstate),
                Event::MouseWheel(_, TouchPhase::Started) => Some(Message::StartWheel),
                Event::MouseWheel(_, TouchPhase::Ended) => Some(Message::EndWheel),
                Event::MouseWheel(_, TouchPhase::Cancelled) => Some(Message::EndWheel),
                Event::MouseWheel(MouseScrollDelta::PixelDelta(_, dy), TouchPhase::Moved) => Some(Message::MoveWheel(dy)),
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
