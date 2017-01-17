pub mod color;
pub mod dock_top;
pub mod dock_left;
pub mod spectrum;

use traveller::Traveller;
use caravel::dock_top::DockTopCaravel;
use caravel::dock_left::DockLeftCaravel;
use std::marker::Sized;
use vrcounter::sigil::Sigil;

pub trait Caravel {
    fn embark(&self) -> Traveller;

    fn dock_top<TopC>(self, top_units: f32, top_caravel: TopC) -> DockTopCaravel<Self, TopC>
        where TopC: Caravel, Self: Sized
    {
        DockTopCaravel::new(top_units, self, top_caravel)
    }

    fn dock_left<LeftC>(self, left_units: f32, left_caravel: LeftC) -> DockLeftCaravel<LeftC, Self>
        where LeftC: Caravel, Self: Sized
    {
        DockLeftCaravel::new(left_units, left_caravel, self)
    }
}

pub fn ids_from_sigil(sigil: &Sigil) -> Vec<u64> {
    use rand;
    if sigil.is_fill() {
        vec![rand::random::<u64>()]
    } else {
        let mut ids = Vec::new();
        for _ in 0..sigil.strokes.len() {
            ids.push(rand::random::<u64>());
        }
        ids
    }
}
