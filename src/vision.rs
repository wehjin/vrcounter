use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use patch::Patch;
use mist::Mist;
use beat::Beat;
use vision;

#[derive(Copy, Clone)]
pub enum Outcome {
    Tick,
}

pub struct Vision<Msg> {
    pub vision_message_adapter: Rc<Fn(vision::Outcome) -> Msg>,
    pub patches: HashMap<u64, Patch>,
    pub mists: HashMap<u64, Mist>,
    beats: HashMap<u64, Beat>,
}

impl<Msg> Vision<Msg> {
    pub fn create<F>(adapter: F) -> Self where F: Fn(vision::Outcome) -> Msg + 'static {
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
    pub fn add_mist(&mut self, mist: Mist) {
        self.mists.insert(mist.id(), mist);
    }
    pub fn find_mists(&self, x: f32, y: f32, z: f32) -> Vec<&Mist> {
        let mut mists = Vec::new();
        for (_, it) in &self.mists {
            let mist: &Mist = it;
            if mist.contains(x, y, z) {
                mists.push(mist);
            }
        }
        mists
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

    #[test]
    fn find_mists() {
        use cage::Cage;
        use mist::Mist;

        let mut vision = Vision::create(|x| x);
        vision.add_mist(Default::default());
        vision.add_mist(Mist::new(10, Cage::from((-10.0, -9.0, -1.0, 1.0, -1.0, 1.0))));
        let mists = vision.find_mists(0.0, 0.0, 0.0);
        assert_eq!(1, mists.len());
    }

    #[test]
    fn find_beats() {
        use beat::Beat;
        use std::time::{Duration, Instant};

        let mut vision = Vision::create(|x| x);
        let now = Instant::now();
        let beat = Beat::until_instant(1, now + Duration::from_millis(3000));
        vision.add_beat(beat);

        assert_eq!(1, vision.find_beats(&now).len());
        let future = now + Duration::from_millis(10000);
        assert_eq!(0, vision.find_beats(&future).len());
    }
}

