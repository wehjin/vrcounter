use traveller::Traveller;
use caravel::color::ColorCaravel;
use caravel::lambda::LambdaCaravel;
use caravel::proxy::ProxyCaravel;
use vrcounter::sigil::Sigil;
use journal::Journal;
use std::rc::Rc;
use std::fmt::Write;
use screen_metrics::ScreenMetrics;
use vrcounter::color::*;
use caravel::spectrum::SpectrumCaravel;
use vrcounter::sakura::{PressLabel, AsciiPoint};
use caravel::Caravel;

enum MidlineSide {
    None,
    Left,
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

pub fn line_editor(line: &str, _: usize, _: char, color: [f32; 4]) -> LambdaCaravel {
    let init_preline: String = String::from(line);
    LambdaCaravel::new(move || {
        let mut preline: String = String::from(init_preline.as_str());
        let mut midline: String = String::new();
        let mut midline_side = MidlineSide::None;
        let mut cursor_color_index: usize = 0;
        let mut insertion_time: u64 = 0;
        let mut back_erasure_time: u64 = 0;
        let mut selection_expand_left_time: u64 = 0;

        Traveller::Lambda {
            on_travel: Box::new(move |shared_journal: Rc<Journal>| {
                let journal: &Journal = shared_journal.as_ref();

                let optional_preview_press = if journal.find_press(PressLabel::Ascii(AsciiPoint::Y), 0) {
                    Some(PressLabel::Ascii(AsciiPoint::Y))
                } else {
                    None
                };

                let should_erase_back = journal.find_press(PressLabel::Ascii(AsciiPoint::Backspace), back_erasure_time);
                if should_erase_back {
                    if midline.is_empty() {
                        preline.pop();
                    } else {
                        midline.clear();
                        midline_side = MidlineSide::None;
                    }
                    back_erasure_time = journal.time();
                }

                let should_insert = journal.find_press(PressLabel::Ascii(AsciiPoint::Space), insertion_time);
                if should_insert {
                    if let Some(PressLabel::Ascii(ascii_point)) = optional_preview_press {
                        preline.write_char(ascii_point.as_char()).unwrap();
                    } else {
                        preline.write_char(' ').unwrap();
                    }
                    midline.clear();
                    midline_side = MidlineSide::None;
                    insertion_time = journal.time();
                }

                let should_expand_selection_left = journal.find_press(PressLabel::SelectionEditLeft, selection_expand_left_time);
                if should_expand_selection_left {
                    if let Some(c) = preline.pop() {
                        midline.insert(0, c);
                        midline_side = MidlineSide::Left;
                    }
                    selection_expand_left_time = journal.time();
                }

                let screen_metrics: ScreenMetrics = journal.screen_metrics();
                let line_height = screen_metrics.active_cage.frame.h;

                let (preline_units, preline_caravel) = {
                    if preline.is_empty() {
                        (0.0, ColorCaravel::new(Sigil::of_fill(), ROSE))
                    } else {
                        let sigil = Sigil::of_line(preline.as_str(), journal.glyffiary());
                        let preline_width = sigil.width_per_height() * line_height;
                        let (preline_units, _) = screen_metrics.main_units_to_grid(preline_width, line_height);
                        (preline_units, ColorCaravel::new(sigil, color))
                    }
                };

                let (cursor_width, cursor_caravel) = {
                    if let Some(PressLabel::Ascii(ascii_point)) = optional_preview_press {
                        let sigil = Sigil::of_point(ascii_point.as_char(), journal.glyffiary());
                        cursor_width_and_caravel(sigil, line_height, &screen_metrics, cursor_color_index)
                    } else {
                        if midline.len() > 0 {
                            let sigil = Sigil::of_line(&midline, journal.glyffiary());
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
