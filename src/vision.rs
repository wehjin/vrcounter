use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use patch::Patch;
use mist::Mist;
use beat::Beat;

#[derive(Copy, Clone)]
pub enum VisionMessage {
    Tick,
}

pub struct Vision<Msg> {
    pub vision_message_adapter: Rc<Fn(VisionMessage) -> Msg>,
    pub patches: HashMap<u64, Patch>,
    pub mists: HashMap<u64, Mist>,
    beats: HashMap<u64, Beat>,
}

impl<Msg> Vision<Msg> {
    pub fn create<F>(adapter: F) -> Self where F: Fn(VisionMessage) -> Msg + 'static {
        Vision {
            vision_message_adapter: Rc::new(adapter),
            patches: HashMap::new(),
            mists: HashMap::new(),
            beats: HashMap::new(),
        }
    }
    pub fn add_patch(&mut self, patch: Patch) {
        self.patches.insert(patch.id, patch);
    }
    pub fn add_beat(&mut self, beat: Beat) {
        self.beats.insert(beat.id(), beat);
    }
    pub fn find_beats(&self, instant: &Instant) -> Vec<&Beat> {
        let mut beats = Vec::new();
        for (_, beat) in &self.beats {
            if beat.contains(instant) {
                beats.push(beat);
            }
        }
        beats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beat::Beat;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

    enum Message {
        Tish,
    }

    #[test]
    fn find_beats() {
        let mut vision = Vision::new(Rc::new(move |vision_message: VisionMessage| -> Message {
            match vision_message {
                VisionMessage::Tick => Message::Tish,
            }
        }));

        let now = Instant::now();
        let beat = Beat::until_instant(1, now + Duration::from_millis(3000));
        vision.add_beat(beat);

        assert_eq!(1, vision.find_beats(&now).len());
        let future = now + Duration::from_millis(10000);
        assert_eq!(0, vision.find_beats(&future).len());
    }
}

