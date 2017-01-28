use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AsciiPoint {
    Nul,
    Space,
    Backspace,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z
}

impl From<f32> for AsciiPoint {
    fn from(value: f32) -> Self {
        let ascii = value.abs().min(255.0) as u8;
        match ascii {
            65 => AsciiPoint::A,
            66 => AsciiPoint::B,
            67 => AsciiPoint::C,
            68 => AsciiPoint::D,
            69 => AsciiPoint::E,
            70 => AsciiPoint::F,
            71 => AsciiPoint::G,
            72 => AsciiPoint::H,
            73 => AsciiPoint::I,
            74 => AsciiPoint::J,
            75 => AsciiPoint::K,
            76 => AsciiPoint::L,
            77 => AsciiPoint::M,
            78 => AsciiPoint::N,
            79 => AsciiPoint::O,
            80 => AsciiPoint::P,
            81 => AsciiPoint::Q,
            82 => AsciiPoint::R,
            83 => AsciiPoint::S,
            84 => AsciiPoint::T,
            85 => AsciiPoint::U,
            86 => AsciiPoint::V,
            87 => AsciiPoint::W,
            88 => AsciiPoint::X,
            89 => AsciiPoint::Y,
            90 => AsciiPoint::Z,
            _ => AsciiPoint::Nul,
        }
    }
}

impl AsciiPoint {
    pub fn as_char(&self) -> char {
        match self {
            &AsciiPoint::Nul => '\x00',
            &AsciiPoint::Backspace => '\x08',
            &AsciiPoint::Space => ' ',
            &AsciiPoint::A => 'A',
            &AsciiPoint::B => 'B',
            &AsciiPoint::C => 'C',
            &AsciiPoint::D => 'D',
            &AsciiPoint::E => 'E',
            &AsciiPoint::F => 'F',
            &AsciiPoint::G => 'G',
            &AsciiPoint::H => 'H',
            &AsciiPoint::I => 'I',
            &AsciiPoint::J => 'J',
            &AsciiPoint::K => 'K',
            &AsciiPoint::L => 'L',
            &AsciiPoint::M => 'M',
            &AsciiPoint::N => 'N',
            &AsciiPoint::O => 'O',
            &AsciiPoint::P => 'P',
            &AsciiPoint::Q => 'Q',
            &AsciiPoint::R => 'R',
            &AsciiPoint::S => 'S',
            &AsciiPoint::T => 'T',
            &AsciiPoint::U => 'U',
            &AsciiPoint::V => 'V',
            &AsciiPoint::W => 'W',
            &AsciiPoint::X => 'X',
            &AsciiPoint::Y => 'Y',
            &AsciiPoint::Z => 'Z',
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PressLabel {
    Ascii(AsciiPoint),
    SelectionEditLeft,
}

#[derive(Copy, Clone)]
pub enum PressConclusion {
    Available,
    Claimed,
    Taken,
    Expired,
}

#[derive(Copy, Clone)]
pub struct Press {
    press_time: u64,
    label: PressLabel,
    release_time: Option<u64>,
    conclusion: Option<PressConclusion>
}

impl Press {
    pub fn set_release_time(&mut self, time: u64) {
        self.release_time = Some(time);
    }
    pub fn press_time(&self) -> u64 {
        self.press_time
    }
    pub fn is_released(&self) -> bool {
        if let Some(release_time) = self.release_time {
            release_time >= self.press_time
        } else {
            false
        }
    }
    pub fn starts_after_time(&self, time: u64) -> bool {
        self.press_time > time
    }
}

pub struct Pressboard {
    presses: HashMap<PressLabel, Press>,
    optional_preview: Option<AsciiPoint>,
}

impl Pressboard {
    pub fn new() -> Self {
        Pressboard { presses: HashMap::new(), optional_preview: None }
    }
    pub fn set_preview_option(&mut self, preview: Option<AsciiPoint>) {
        self.optional_preview = preview
    }
    pub fn preview_option(&self) -> Option<AsciiPoint> {
        self.optional_preview
    }
    pub fn begin_press(&mut self, label: PressLabel, time: u64) {
        let next_press = match self.presses.get(&label) {
            None => Press { press_time: time, label: label, release_time: None, conclusion: None },
            Some(press) => {
                let mut next_press: Press = *press;
                if next_press.release_time.is_some() {
                    next_press.press_time = time;
                    next_press.release_time = None;
                }
                next_press
            },
        };
        self.presses.insert(label, next_press);
    }
    pub fn end_press(&mut self, label: PressLabel, time: u64) {
        if let Some(press) = self.presses.get_mut(&label) {
            press.set_release_time(time);
        }
    }
    pub fn find_press(&self, label: PressLabel, time: u64) -> bool {
        if self.find_matching_preview(label) {
            true
        } else if let Some(press) = self.presses.get(&label) {
            press.starts_after_time(time) && !press.is_released()
        } else {
            false
        }
    }
    fn find_matching_preview(&self, label: PressLabel) -> bool {
        if let PressLabel::Ascii(asciipoint) = label {
            if let Some(preview_asciipoint) = self.optional_preview {
                asciipoint == preview_asciipoint
            } else {
                false
            }
        } else {
            false
        }
    }
}
