use screen_metrics::ScreenMetrics;
use vrcounter::Patch;
use std::collections::HashMap;
use cage::Cage;

pub trait Journal {
    fn screen_metrics(&self) -> ScreenMetrics;
    fn set_patch(&mut self, patch_id: u64, patch: Patch);
}

pub struct CageJournal<'a, J: Journal + 'a> {
    cage: Cage,
    delegate: &'a mut J,
}

impl<'a, J: Journal> Journal for CageJournal<'a, J> {
    fn screen_metrics(&self) -> ScreenMetrics {
        self.delegate.screen_metrics().with_active_cage(self.cage)
    }

    fn set_patch(&mut self, patch_id: u64, patch: Patch) {
        self.delegate.set_patch(patch_id, patch);
    }
}

impl<'a, J: Journal> CageJournal<'a, J> {
    pub fn new(cage: Cage, delegate: &'a mut J) -> Self {
        CageJournal { cage: cage, delegate: delegate }
    }
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
