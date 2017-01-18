pub mod color;
pub mod dock_top;
pub mod dock_left;
pub mod spectrum;
pub mod lambda;

use traveller::Traveller;
use caravel::dock_top::DockTopCaravel;
use caravel::dock_left::DockLeftCaravel;
use caravel::lambda::LambdaCaravel;
use std::marker::Sized;
use vrcounter::sigil::Sigil;
use journal::Journal;
use std::rc::Rc;
use std::marker::Send;
use std::marker::Sync;
use std::boxed::Box;

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

    fn contract(self, left_right_units: f32, bottom_top_units: f32) -> LambdaCaravel
        where Self: Sized + Send + Sync + 'static
    {
        LambdaCaravel::new(move || {
            let mut traveller = self.embark();
            Traveller::Lambda {
                on_travel: Box::new(move |shared_journal: Rc<Journal>| {
                    let journal = Journal::new_with_contraction(&shared_journal, left_right_units, bottom_top_units);
                    traveller.travel(Rc::new(journal));
                })
            }
        })
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
