use journal::Journal;
use vrcounter::color::SPECTRUM;
use std::boxed::Box;
use std::rc::Rc;
use cage::Translation;
use cage::Cage;
use vrcounter::Patch;
use vrcounter::PatchPosition;
use vrcounter::patch::FILL_POINT;
use vrcounter::sigil::{Sigil, Stroke};

pub enum Traveller {
    Lambda {
        on_travel: Box<FnMut(Rc<Journal>) -> ()>
    },
    Color {
        ids: Vec<u64>,
        color: [f32; 4],
        sigil: Sigil,
    },
    Spectrum {
        ids: Vec<u64>,
        color_index: usize,
        sigil: Sigil,
    },
    DockTop {
        top_units: f32,
        bottom_traveller: Box<Traveller>,
        top_traveller: Box<Traveller>,
    },
    DockLeft {
        left_units: f32,
        left_traveller: Box<Traveller>,
        right_traveller: Box<Traveller>,
    },
}

impl Traveller {
    pub fn travel(&mut self, journal: Rc<Journal>) {
        match self {
            &mut Traveller::Lambda { ref mut on_travel } => {
                on_travel(journal)
            },
            &mut Traveller::Color { ref ids, color, ref sigil } => {
                let cage = journal.screen_metrics().active_cage;
                let patches = patches_from_sigil(&sigil, &cage, color, &ids);
                for patch in patches {
                    journal.set_patch(patch.id, patch);
                }
            },
            &mut Traveller::Spectrum { ref ids, ref mut color_index, ref sigil } => {
                let cage = journal.screen_metrics().active_cage;
                let spectrum_index = *color_index % SPECTRUM.len();
                let color = SPECTRUM[spectrum_index];
                let patch = patches_from_sigil(sigil, &cage, color, ids)[0];
                journal.set_patch(patch.id, patch);
                *color_index = *color_index + 1;
            },
            &mut Traveller::DockTop { top_units, ref mut bottom_traveller, ref mut top_traveller } => {
                let screen_metrics = journal.screen_metrics();
                let cage = screen_metrics.active_cage;
                let top_height = screen_metrics.preferred_reading_height * top_units;
                let (top_cage, bottom_cage) = divide_cage_at_top(cage, top_height);
                {
                    let bottom_journal = Journal::Cage { cage: bottom_cage, delegate: journal.clone() };
                    bottom_traveller.travel(Rc::new(bottom_journal));
                }
                {
                    let top_journal = Journal::Cage { cage: top_cage, delegate: journal.clone() };
                    top_traveller.travel(Rc::new(top_journal));
                }
            }
            &mut Traveller::DockLeft { left_units, ref mut left_traveller, ref mut right_traveller } => {
                let screen_metrics = journal.screen_metrics();
                let cage = screen_metrics.active_cage;
                let left_width = screen_metrics.preferred_reading_height * left_units;
                let (left_cage, right_cage) = divide_cage_at_left(cage, left_width);
                {
                    let journal = Journal::Cage { cage: left_cage, delegate: journal.clone() };
                    left_traveller.travel(Rc::new(journal));
                }
                {
                    let journal = Journal::Cage { cage: right_cage, delegate: journal.clone() };
                    right_traveller.travel(Rc::new(journal));
                }
            }
        }
    }
}

fn divide_cage_at_top(cage: Cage, top_height: f32) -> (Cage, Cage) {
    let bottom_height = cage.frame.h - top_height;
    let bottom_cage = cage.translate_sides(Translation { top: -top_height, ..Default::default() });
    let top_cage = cage.translate_sides(Translation { bottom: bottom_height, ..Default::default() });
    (top_cage, bottom_cage)
}

fn divide_cage_at_left(cage: Cage, left_width: f32) -> (Cage, Cage) {
    let right_width = cage.frame.w - left_width;
    let left_cage = cage.translate_sides(Translation { right: -right_width, ..Default::default() });
    let right_cage = cage.translate_sides(Translation { left: left_width, ..Default::default() });
    (left_cage, right_cage)
}

fn patch_from_stroke(stroke: &Stroke, cage: &Cage, color: [f32; 4], id: u64) -> Patch {
    let glyph = stroke.ascii_point();
    let cage_height = cage.frame.h;
    let stroke_cage = stroke.cage();
    let (stroke_left, stroke_right, _, _, _, _) = stroke_cage.limits();
    let patch_cage = cage.translate_sides(Translation {
        left: stroke_left * cage_height,
        right: -(cage.frame.w - stroke_right * cage_height),
        ..Default::default()
    });
    let position = PatchPosition::from_cage(&patch_cage);
    Patch { id: id, glyph: glyph, color: color, position: position }
}

fn patches_from_sigil(sigil: &Sigil, cage: &Cage, color: [f32; 4], ids: &Vec<u64>) -> Vec<Patch> {
    if sigil.is_fill() {
        let patch = Patch { id: ids[0], glyph: FILL_POINT, color: color, position: PatchPosition::from_cage(&cage) };
        vec![patch]
    } else {
        let mut patches = Vec::new();
        for (i, stroke) in sigil.strokes.iter().enumerate() {
            let id = ids[i];
            let patch = patch_from_stroke(&stroke, cage, color, id);
            patches.push(patch)
        }
        patches
    }
}
