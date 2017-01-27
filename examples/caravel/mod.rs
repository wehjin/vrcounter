pub mod color;
pub mod dock_top;
pub mod dock_left;
pub mod lambda;
pub mod spectrum;
pub mod proxy;
pub mod line_editor;

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
use cage::Cage;
use cage::Translation;

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

    fn dock_far<FarC>(self, far_units: f32, far_caravel: FarC) -> LambdaCaravel
        where FarC: Caravel + Send + Sync + 'static,
              Self: Sized + Send + Sync + 'static,
    {
        LambdaCaravel::new(move || {
            let mut far_traveller = far_caravel.embark();
            let mut near_traveller = self.embark();
            Traveller::Lambda {
                on_travel: Box::new(move |journal: Rc<Journal>| {
                    let screen_metrics = journal.screen_metrics();
                    let cage = screen_metrics.active_cage;
                    let far_depth = screen_metrics.preferred_z_increment * far_units;
                    let (far_cage, near_cage) = divide_cage_at_far(cage, far_depth);
                    {
                        let far_journal = Journal::Cage { cage: far_cage, delegate: journal.clone() };
                        far_traveller.travel(Rc::new(far_journal));
                    }
                    {
                        let near_journal = Journal::Cage { cage: near_cage, delegate: journal.clone() };
                        near_traveller.travel(Rc::new(near_journal));
                    }
                })
            }
        })
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

fn divide_cage_at_far(cage: Cage, far_depth: f32) -> (Cage, Cage) {
    let far_cage = cage;
    let near_cage = cage.translate_sides(Translation { far: far_depth, near: far_depth, ..Default::default() });
    (far_cage, near_cage)
}
