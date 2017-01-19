use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AsciiPoint {
    Y,
    U
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PressLabel {
    Ascii(AsciiPoint)
}

pub enum PressConclusion {
    Available,
    Claimed,
    Taken,
    Expired,
}

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
    pub fn is_unreleased(&self) -> bool {
        self.release_time.is_none()
    }
}

pub struct Pressboard {
    presses: HashMap<PressLabel, Press>
}

impl Pressboard {
    pub fn new() -> Self {
        Pressboard { presses: HashMap::new() }
    }
    pub fn begin_press(&mut self, label: PressLabel, time: u64) {
        let press = Press { press_time: time, label: label, release_time: None, conclusion: None };
        self.presses.insert(label, press);
    }
    pub fn end_press(&mut self, label: PressLabel, time: u64) {
        if let Some(press) = self.presses.get_mut(&label) {
            press.set_release_time(time);
        }
    }
    pub fn find_press(&self, label: PressLabel, time: u64) -> bool {
        if let Some(press) = self.presses.get(&label) {
            (press.press_time() > time) && press.is_unreleased()
        } else {
            false
        }
    }
}
