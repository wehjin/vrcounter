use os::is_windows;
use glium::glutin::{Event, ElementState};

pub enum Key {
    LookUp,
    LookDown,
    LookRight,
    LookLeft,
    LookFar,
    LookNear,
    ResetLook,
    H,
    J,
    K,
    L,
    RBracket,
    LBracket,
    Quit,
}

impl Key {
    fn for_windows(glutin_event: &Event) -> Option<Self> {
        match glutin_event {
            &Event::Closed => Some(Key::Quit),
            &Event::KeyboardInput(ElementState::Pressed, 1, _) => Some(Key::Quit),
            &Event::KeyboardInput(ElementState::Pressed, 52, _) => Some(Key::ResetLook),
            &Event::KeyboardInput(ElementState::Pressed, 30, _) => Some(Key::LookLeft),
            &Event::KeyboardInput(ElementState::Pressed, 32, _) => Some(Key::LookRight),
            &Event::KeyboardInput(ElementState::Pressed, 17, _) => Some(Key::LookUp),
            &Event::KeyboardInput(ElementState::Pressed, 31, _) => Some(Key::LookDown),
            &Event::KeyboardInput(ElementState::Pressed, 16, _) => Some(Key::LookFar),
            &Event::KeyboardInput(ElementState::Pressed, 18, _) => Some(Key::LookNear),
            &Event::KeyboardInput(ElementState::Pressed, code, _) => {
                println!("{}", code);
                None
            },
            _ => None
        }
    }
    fn for_mac(glutin_event: &Event) -> Option<Self> {
        match glutin_event {
            &Event::Closed => Some(Key::Quit),
            &Event::KeyboardInput(ElementState::Pressed, 53, _) => Some(Key::Quit),
            &Event::KeyboardInput(ElementState::Pressed, 47, _) => Some(Key::ResetLook),
            &Event::KeyboardInput(ElementState::Pressed, 1, _) => Some(Key::LookDown),
            &Event::KeyboardInput(ElementState::Pressed, 13, _) => Some(Key::LookUp),
            &Event::KeyboardInput(ElementState::Pressed, 0, _) => Some(Key::LookLeft),
            &Event::KeyboardInput(ElementState::Pressed, 2, _) => Some(Key::LookRight),
            &Event::KeyboardInput(ElementState::Pressed, 12, _) => Some(Key::LookFar),
            &Event::KeyboardInput(ElementState::Pressed, 14, _) => Some(Key::LookNear),
            &Event::KeyboardInput(ElementState::Pressed, 4, _) => Some(Key::H),
            &Event::KeyboardInput(ElementState::Pressed, 38, _) => Some(Key::J),
            &Event::KeyboardInput(ElementState::Pressed, 40, _) => Some(Key::K),
            &Event::KeyboardInput(ElementState::Pressed, 37, _) => Some(Key::L),
            &Event::KeyboardInput(ElementState::Pressed, 30, _) => Some(Key::RBracket),
            &Event::KeyboardInput(ElementState::Pressed, 33, _) => Some(Key::LBracket),
            &Event::KeyboardInput(ElementState::Pressed, code, _) => {
                println!("{}", code);
                None
            },
            _ => None
        }
    }
}

pub struct Keymap {
    is_windows: bool,
}

impl Keymap {
    pub fn init() -> Self {
        Keymap { is_windows: is_windows() }
    }
    pub fn key_for_event(&self, glutin_event: &Event) -> Option<Key> {
        if self.is_windows {
            Key::for_windows(glutin_event)
        } else {
            Key::for_mac(glutin_event)
        }
    }
}