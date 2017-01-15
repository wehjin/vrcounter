pub mod color;
pub mod spectrum;
pub mod dock_top;
pub mod dock_left;

use journal::Journal;

pub trait Traveller {
    fn travel<J: Journal>(&mut self, journal: &mut J);
}
