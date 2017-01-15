mod prime;
mod cage;

use screen_metrics::ScreenMetrics;
use vrcounter::Patch;

pub trait Journal {
    fn screen_metrics(&self) -> ScreenMetrics;
    fn set_patch(&mut self, patch_id: u64, patch: Patch);
}

pub use journal::prime::PrimeJournal;
pub use journal::cage::CageJournal;
