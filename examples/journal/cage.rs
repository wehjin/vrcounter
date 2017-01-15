use screen_metrics::ScreenMetrics;
use vrcounter::Patch;
use cage::Cage;
use journal::Journal;

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
