pub mod color;
pub mod spectrum;

use journal::Journal;

pub trait Traveller {
    fn travel<J: Journal>(&mut self, journal: &mut J);
}
