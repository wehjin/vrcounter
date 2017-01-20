pub mod color;
pub mod dock_top;
pub mod dock_left;
pub mod lambda;
pub mod spectrum;
pub mod proxy;

use traveller::Traveller;
use caravel::dock_top::DockTopCaravel;
use caravel::dock_left::DockLeftCaravel;
use caravel::color::ColorCaravel;
use caravel::lambda::LambdaCaravel;
use caravel::proxy::ProxyCaravel;
use std::marker::Sized;
use vrcounter::sigil::Sigil;
use journal::Journal;
use std::rc::Rc;
use std::marker::Send;
use std::marker::Sync;

pub fn new_line_editor(line: &str, _: usize, _: char, color: [f32; 4]) -> LambdaCaravel {
    let init_preline: String = String::from(line);
    LambdaCaravel::new(move || {
        let preline: String = String::from(init_preline.as_str());
        let mut cursor_color_index: usize = 0;

        Traveller::Lambda {
            on_travel: Box::new(move |shared_journal: Rc<Journal>| {
                use screen_metrics::ScreenMetrics;
                use vrcounter::color::*;
                use caravel::Caravel;
                use caravel::spectrum::SpectrumCaravel;
                use vrcounter::sakura::{PressLabel, AsciiPoint};

                let sigil = Sigil::of_line(preline.as_str(), shared_journal.glyffiary());
                let screen_metrics: ScreenMetrics = shared_journal.screen_metrics();
                let preline_height = screen_metrics.active_cage.frame.h;
                let preline_width = sigil.width_per_height() * preline_height;
                let (preline_units, _) = screen_metrics.main_units_to_grid(preline_width, preline_height);
                let preline_caravel = ColorCaravel::new(sigil, color);

                let (cursor_width, cursor_caravel) = {
                    let journal: &Journal = shared_journal.as_ref();
                    if journal.find_press(PressLabel::Ascii(AsciiPoint::Y), 0) {
                        let sigil = Sigil::of_point('y', journal.glyffiary());
                        let width = sigil.width_per_height() * preline_height;
                        let (width_units, _) = screen_metrics.main_units_to_grid(width, 0.0);
                        let caravel = ProxyCaravel::new(ColorCaravel::new(sigil, WHITE));
                        (width_units, caravel)
                    } else {
                        let caravel = ProxyCaravel::new(SpectrumCaravel::new(cursor_color_index));
                        (0.4, caravel)
                    }
                };
                let caravel = ColorCaravel::new(Sigil::of_fill(), GREY_01)
                    .dock_left(cursor_width, cursor_caravel)
                    .dock_left(preline_units, preline_caravel);

                let mut traveller = caravel.embark();
                traveller.travel(shared_journal);
                cursor_color_index = cursor_color_index + 1;
            })
        }
    })
}

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
