use glium::glutin::{Event, ElementState};

pub enum Key {
    LookUp,
    LookDown,
    LookRight,
    LookLeft,
    StepBack,
    StepForward,
    ResetLook,
    HandLeft,
    HandDown,
    HandUp,
    HandRight,
    HandForward,
    HandBack,
    Quit,
}

pub struct Keymap {}

impl Keymap {
    pub fn init() -> Self {
        Keymap {}
    }
    pub fn key_for_event(&self, glutin_event: &Event) -> Option<Key> {
        use glium::glutin::VirtualKeyCode;
        match glutin_event {
            &Event::Closed => Some(Key::Quit),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => Some(Key::Quit),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Q)) => Some(Key::StepForward),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Z)) => Some(Key::StepBack),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::H)) => Some(Key::HandLeft),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::J)) => Some(Key::HandDown),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::K)) => Some(Key::HandUp),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::L)) => Some(Key::HandRight),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::RBracket)) => Some(Key::HandForward),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::LBracket)) => Some(Key::HandBack),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::S)) => Some(Key::LookDown),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::W)) => Some(Key::LookUp),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => Some(Key::LookLeft),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::D)) => Some(Key::LookRight),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Period)) => Some(Key::ResetLook),
            &Event::KeyboardInput(ElementState::Pressed, _, Some(virtual_key_code)) => {
                println!("Virtual key {:?}", virtual_key_code);
                None
            },
            _ => None,
        }
    }
}