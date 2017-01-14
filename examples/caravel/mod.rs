pub mod color;
pub mod top_dock;
pub mod spectrum;

use traveller::Traveller;
use caravel::top_dock::TopDockCaravel;
use std::marker::Sized;

pub trait Caravel<T: Traveller> {
    fn embark(&self) -> T;

    fn dock_top<TopT, TopC>(self, top_units: f32, top_caravel: TopC) -> TopDockCaravel<T, Self, TopT, TopC>
        where TopT: Traveller, TopC: Caravel<TopT>, Self: Sized
    {
        TopDockCaravel::new(top_units, self, top_caravel)
    }
}
