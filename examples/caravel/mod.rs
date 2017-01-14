pub mod color;
pub mod top_dock;

use traveller::Traveller;

pub trait Caravel<T: Traveller> {
    fn embark(&self) -> T;
}
