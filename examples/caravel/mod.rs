pub mod color;
pub mod dock_top;
pub mod dock_left;
pub mod spectrum;

use traveller::Traveller;
use caravel::dock_top::DockTopCaravel;
use caravel::dock_left::DockLeftCaravel;
use std::marker::Sized;

pub trait Caravel<T: Traveller> {
    fn embark(&self) -> T;

    fn dock_top<TopT, TopC>(self, top_units: f32, top_caravel: TopC) -> DockTopCaravel<T, Self, TopT, TopC>
        where TopT: Traveller, TopC: Caravel<TopT>, Self: Sized
    {
        DockTopCaravel::new(top_units, self, top_caravel)
    }

    fn dock_left<LeftT, LeftC>(self, left_units: f32, left_caravel: LeftC) -> DockLeftCaravel<LeftT, LeftC, T, Self>
        where LeftT: Traveller, LeftC: Caravel<LeftT>, Self: Sized
    {
        DockLeftCaravel::new(left_units, left_caravel, self)
    }
}
