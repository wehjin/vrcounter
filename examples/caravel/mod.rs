pub mod color;

use traveller::Traveller;

pub trait Caravel<T: Traveller> {
    fn embark(&self) -> T;
}
