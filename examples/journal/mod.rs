use screen_metrics::ScreenMetrics;
use vrcounter::Patch;
use std::collections::HashMap;
use cage::Cage;
use std::cell::RefCell;
use std::rc::Rc;

pub enum Journal {
    Prime {
        screen_metrics: ScreenMetrics,
        patches: RefCell<HashMap<u64, Patch>>,
    },
    Cage {
        cage: Cage,
        delegate: Rc<Journal>,
    }
}

impl Journal {
    pub fn screen_metrics(&self) -> ScreenMetrics {
        match self {
            &Journal::Prime { ref screen_metrics, .. } => {
                *screen_metrics
            },
            &Journal::Cage { ref cage, ref delegate } => {
                delegate.screen_metrics().with_active_cage(*cage)
            }
        }
    }
    pub fn set_patch(&self, patch_id: u64, patch: Patch) {
        match self {
            &Journal::Prime { ref patches, .. } => {
                let mut edit = patches.borrow_mut();
                edit.insert(patch_id, patch);
            },
            &Journal::Cage { ref delegate, .. } => {
                delegate.set_patch(patch_id, patch);
            }
        }
    }
    pub fn patches(&self) -> HashMap<u64, Patch> {
        match self {
            &Journal::Prime { ref patches, .. } => {
                patches.borrow().clone()
            },
            &Journal::Cage { ref delegate, .. } => {
                delegate.patches()
            }
        }
    }
}
