use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AsciiPoint {
    Backspace,
    Y,
    U,
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
    presses: HashMap<PressLabel, Press>
}

impl Pressboard {
    pub fn new() -> Self {
        Pressboard { presses: HashMap::new() }
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
        if let Some(press) = self.presses.get(&label) {
            press.starts_after_time(time) && !press.is_released()
        } else {
            false
        }
    }
}
