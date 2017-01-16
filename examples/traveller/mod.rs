pub mod color;
pub mod spectrum;
pub mod dock_top;
pub mod dock_left;

use journal::Journal;

pub trait Traveller {
    fn travel<J: Journal>(&mut self, journal: &mut J);
}

use cage::Translation;
use cage::Cage;
use vrcounter::Patch;
use vrcounter::PatchPosition;
use vrcounter::patch::FILL_POINT;
use vrcounter::sigil::{Sigil, Stroke};

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

pub fn patches_from_sigil(sigil: &Sigil, cage: &Cage, color: [f32; 4], ids: &Vec<u64>) -> Vec<Patch> {
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
