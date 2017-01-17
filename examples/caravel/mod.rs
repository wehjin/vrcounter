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
use cage::Translation;
use std::sync::Arc;
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
        LambdaCaravel::new(Arc::new(move || {
            let mut center_traveller = self.embark();
            Traveller::Lambda {
                on_travel: Box::new(move |shared_journal: Rc<Journal>| {
                    use screen_metrics::ScreenMetrics;
                    let screen_metrics: ScreenMetrics = shared_journal.screen_metrics();
                    let bottom_top_delta = screen_metrics.preferred_reading_height * bottom_top_units;
                    let left_right_delta = screen_metrics.preferred_reading_height * left_right_units;
                    let contract_cage = screen_metrics.active_cage.translate_sides(
                        Translation {
                            bottom: bottom_top_delta, top: -bottom_top_delta,
                            left: left_right_delta, right: -left_right_delta,
                            ..Default::default()
                        });
                    let contract_journal = Journal::Cage { cage: contract_cage, delegate: shared_journal };
                    center_traveller.travel(Rc::new(contract_journal));
                })
            }
        }))
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
