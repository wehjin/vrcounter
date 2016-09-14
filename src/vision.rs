use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use patch::Patch;
use mist::Mist;
use beat::Beat;
use common::Wish;

#[derive(Clone)]
pub struct Vision<Msg> {
    adapters: HashMap<u64, Rc<Fn(Wish) -> Option<Msg>>>,
    pub patches: HashMap<u64, Patch>,
    pub mists: HashMap<u64, Mist>,
    pub beats: HashMap<u64, Beat>,
}

impl<Msg> Default for Vision<Msg> {
    fn default() -> Self {
        Vision::new()
    }
}

impl<Msg> Vision<Msg> {
    pub fn new() -> Self {
        Vision {
            adapters: HashMap::new(),
            patches: HashMap::new(),
            mists: HashMap::new(),
            beats: HashMap::new(),
        }
    }
    pub fn add_patch(&mut self, patch: Patch) {
        self.patches.insert(patch.id, patch);
    }
    pub fn add_mist<T>(&mut self, mist: Mist, adapter: T) where T: Fn(Wish) -> Option<Msg> + 'static {
        self.adapters.insert(mist.id(), Rc::new(adapter));
        self.mists.insert(mist.id(), mist);
    }
    pub fn add_beat<T>(&mut self, beat: Beat, adapter: T) where T: Fn(Wish) -> Option<Msg> + 'static {
        self.adapters.insert(beat.id(), Rc::new(adapter));
        self.beats.insert(beat.id(), beat);
    }
    pub fn add_vision<T, F>(&mut self, sub_vision: Vision<T>, adapter: F)
        where T: 'static, F: Fn(T) -> Option<Msg> + 'static
    {
        for (id, patch) in sub_vision.patches {
            self.patches.insert(id, patch);
        }
        for (id, mist) in sub_vision.mists {
            self.mists.insert(id, mist);
        }
        for (id, beat) in sub_vision.beats {
            self.beats.insert(id, beat);
        }
        let adapter_rc = Rc::new(adapter);
        for (id, sub_adapter_rc) in sub_vision.adapters {
            let cloned_sub_adapter_rc = sub_adapter_rc.clone();
            let cloned_adapter_rc = adapter_rc.clone();
            let combined_adapter = move |wish| {
                match (*cloned_sub_adapter_rc)(wish) {
                    None => None,
                    Some(sub_message) => (*cloned_adapter_rc)(sub_message),
                }
            };
            self.adapters.insert(id, Rc::new(combined_adapter));
        }
    }
    pub fn get_message_option(&self, id: u64, wish: Wish) -> Option<Msg> {
        let adapter_rc_op = self.adapters.get(&id);
        if let Some(adapter_rc) = adapter_rc_op {
            (*adapter_rc)(wish)
        } else {
            None
        }
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

        let mut vision = Vision::new() as Vision<()>;
        vision.add_mist(Default::default(), |_| None);
        vision.add_mist(Mist::new(10, Cage::from((-10.0, -9.0, -1.0, 1.0, -1.0, 1.0))), |_| None);
        let mists = vision.find_mists(0.0, 0.0, 0.0);
        assert_eq!(1, mists.len());
    }

    #[test]
    fn find_beats() {
        use beat::Beat;
        use std::time::{Duration, Instant};

        let mut vision = Vision::new() as Vision<()>;
        let now = Instant::now();
        let beat = Beat::until_instant(1, now + Duration::from_millis(3000));
        vision.add_beat(beat, |_| None);

        assert_eq!(1, vision.find_beats(&now).len());
        let future = now + Duration::from_millis(10000);
        assert_eq!(0, vision.find_beats(&future).len());
    }
}

