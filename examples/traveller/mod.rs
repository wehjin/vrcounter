pub mod color;
pub mod spectrum;
pub mod top_dock;

use journal::Journal;

pub trait Traveller {
    fn travel<J: Journal>(&mut self, journal: &mut J);
}
