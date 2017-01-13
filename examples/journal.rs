use screen_metrics::ScreenMetrics;
use vrcounter::Patch;
use std::collections::HashMap;

pub trait Journal {
    fn screen_metrics(&self) -> ScreenMetrics;
    fn set_patch(&mut self, patch_id: u64, patch: Patch);
}

pub struct PrimeJournal {
    screen_metrics: ScreenMetrics,
    patches: HashMap<u64, Patch>,
}

impl PrimeJournal {
    pub fn new(screen_metrics: ScreenMetrics) -> Self {
        PrimeJournal {
            screen_metrics: screen_metrics,
            patches: HashMap::new(),
        }
    }
    pub fn patches(&self) -> &HashMap<u64, Patch> {
        &self.patches
    }
}

impl Journal for PrimeJournal {
    fn screen_metrics(&self) -> ScreenMetrics {
        self.screen_metrics
    }
    fn set_patch(&mut self, patch_id: u64, patch: Patch) {
        self.patches.insert(patch_id, patch);
    }
}
