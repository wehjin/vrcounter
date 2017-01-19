use screen_metrics::ScreenMetrics;
use vrcounter::Patch;
use std::collections::HashMap;
use cage::Cage;
use cage::Translation;
use std::cell::RefCell;
use std::rc::Rc;
use vrcounter::glyffin::Glyffiary;
use vrcounter::sakura::{Pressboard, PressLabel};

pub enum Journal {
    Prime {
        screen_metrics: ScreenMetrics,
        patches: RefCell<HashMap<u64, Patch>>,
        shared_glyffiary: Rc<Glyffiary>,
        shared_pressboard: Rc<RefCell<Pressboard>>,
    },
    Cage {
        cage: Cage,
        delegate: Rc<Journal>,
    }
}

impl Journal {
    pub fn find_press(&self, label: PressLabel, time: u64) -> bool {
        match self {
            &Journal::Prime { ref shared_pressboard, .. } => {
                shared_pressboard.borrow().find_press(label, time)
            }
            &Journal::Cage { ref delegate, .. } => {
                delegate.find_press(label, time)
            }
        }
    }


    pub fn glyffiary(&self) -> &Glyffiary {
        match self {
            &Journal::Prime { ref shared_glyffiary, .. } => {
                shared_glyffiary.as_ref()
            },
            &Journal::Cage { ref delegate, .. } => {
                delegate.glyffiary()
            }
        }
    }

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

    pub fn new_with_contraction(shared_journal: &Rc<Self>, left_right_units: f32, bottom_top_units: f32) -> Self {
        let screen_metrics: ScreenMetrics = shared_journal.screen_metrics();
        let (left_right_delta, bottom_top_delta) = screen_metrics.grid_units_to_main(left_right_units, bottom_top_units);
        let new_cage = screen_metrics.active_cage.translate_sides(Translation {
            bottom: bottom_top_delta, top: -bottom_top_delta,
            left: left_right_delta, right: -left_right_delta,
            ..Default::default()
        });
        Journal::Cage { cage: new_cage, delegate: shared_journal.clone() }
    }
}
