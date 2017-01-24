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
use cage::Cage;
use cage::Translation;
use std::fmt::Write;
use screen_metrics::ScreenMetrics;
use vrcounter::color::*;
use caravel::spectrum::SpectrumCaravel;
use vrcounter::sakura::{PressLabel, AsciiPoint};

enum MidlineSide {
    None,
    Left,
    Right
}

pub fn cursor_width_and_caravel(sigil: Sigil,
                                line_height: f32,
                                screen_metrics: &ScreenMetrics,
                                cursor_color_index: usize) -> (f32, ProxyCaravel)
{
    let width = sigil.width_per_height() * line_height;
    let (width_units, _) = screen_metrics.main_units_to_grid(width, 0.0);
    let sigil_on_spectrum = ColorCaravel::new(sigil, GREY_01).dock_far(1.0, SpectrumCaravel::new(cursor_color_index));
    let caravel = ProxyCaravel::new(sigil_on_spectrum);
    (width_units, caravel)
}

pub fn empty_cursor_width_and_caravel(cursor_color_index: usize) -> (f32, ProxyCaravel)
{
    (0.4, ProxyCaravel::new(SpectrumCaravel::new(cursor_color_index)))
}

pub fn new_line_editor(line: &str, _: usize, _: char, color: [f32; 4]) -> LambdaCaravel {
    let init_preline: String = String::from(line);
    LambdaCaravel::new(move || {
        let mut preline: String = String::from(init_preline.as_str());
        let mut midline: String = String::new();
        let mut midline_side = MidlineSide::None;
        let mut cursor_color_index: usize = 0;
        let mut insertion_time: u64 = 0;
        let mut selection_expand_left_time: u64 = 0;

        Traveller::Lambda {
            on_travel: Box::new(move |shared_journal: Rc<Journal>| {
                let optional_ascii_point = if shared_journal.find_press(PressLabel::Ascii(AsciiPoint::Y), 0) {
                    Some('y')
                } else if shared_journal.find_press(PressLabel::Ascii(AsciiPoint::Backspace), 0) {
                    Some('\x08')
                } else {
                    None
                };

                let should_insert = shared_journal.find_press(PressLabel::Ascii(AsciiPoint::U), insertion_time);
                if should_insert {
                    println!("Insert!");
                    if let Some(ascii_char) = optional_ascii_point {
                        if ascii_char != '\x08' {
                            preline.write_char(ascii_char).unwrap();
                        }
                        midline.clear();
                        midline_side = MidlineSide::None;
                    }
                    insertion_time = shared_journal.time();
                }

                let should_expand_selection_left = shared_journal.find_press(PressLabel::SelectionEditLeft, selection_expand_left_time);
                if should_expand_selection_left {
                    if let Some(c) = preline.pop() {
                        midline.insert(0, c);
                        midline_side = MidlineSide::Left;
                    }
                    selection_expand_left_time = shared_journal.time();
                }

                let screen_metrics: ScreenMetrics = shared_journal.screen_metrics();
                let line_height = screen_metrics.active_cage.frame.h;

                let (preline_units, preline_caravel) = {
                    if preline.is_empty() {
                        (0.0, ColorCaravel::new(Sigil::of_fill(), ROSE))
                    } else {
                        let sigil = Sigil::of_line(preline.as_str(), shared_journal.glyffiary());
                        let preline_width = sigil.width_per_height() * line_height;
                        let (preline_units, _) = screen_metrics.main_units_to_grid(preline_width, line_height);
                        (preline_units, ColorCaravel::new(sigil, color))
                    }
                };

                let (cursor_width, cursor_caravel) = {
                    if let Some(ascii_point) = optional_ascii_point {
                        if ascii_point == '\x08' {
                            empty_cursor_width_and_caravel(cursor_color_index)
                        } else {
                            let sigil = Sigil::of_point(ascii_point, shared_journal.glyffiary());
                            cursor_width_and_caravel(sigil, line_height, &screen_metrics, cursor_color_index)
                        }
                    } else {
                        if midline.len() > 0 {
                            let sigil = Sigil::of_line(&midline, shared_journal.glyffiary());
                            cursor_width_and_caravel(sigil, line_height, &screen_metrics, cursor_color_index)
                        } else {
                            empty_cursor_width_and_caravel(cursor_color_index)
                        }
                    }
                };
                let caravel = ColorCaravel::new(Sigil::of_fill(), GREY_01)
                    .dock_left(cursor_width, cursor_caravel)
                    .dock_left(preline_units, preline_caravel);

                let mut traveller = caravel.embark();
                traveller.travel(shared_journal.clone());
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
