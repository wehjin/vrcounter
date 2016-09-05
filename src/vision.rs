use std::collections::HashMap;
use std::rc::Rc;
use patch::Patch;
use mist::Mist;

pub enum VisionMessage {
    Tick,
}

pub struct Vision<Msg> {
    pub vision_message_adapter: Rc<Fn(VisionMessage) -> Msg>,
    pub patches: HashMap<u64, Patch>,
    pub mists: HashMap<u64, Mist>,
}

impl<Msg> Vision<Msg> {
    pub fn new(vision_message_adapter: Rc<Fn(VisionMessage) -> Msg>) -> Self {
        Vision {
            vision_message_adapter: vision_message_adapter.clone(),
            patches: HashMap::new(), mists: HashMap::new()
        }
    }
    pub fn add_patch(&mut self, patch: Patch) {
        self.patches.insert(patch.id, patch);
    }
}

