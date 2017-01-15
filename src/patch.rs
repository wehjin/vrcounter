extern crate cage;

use cage::Cage;
use sigil::Sigil;

#[derive(Debug, Copy, Clone)]
pub struct PatchPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32
}

impl PatchPosition {
    pub fn from_cage(cage: &Cage) -> Self {
        let (left, right, bottom, top, far, near) = cage.limits();
        PatchPosition { left: left, right: right, bottom: bottom, top: top, near: (far + near) / 2.0 }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Patch {
    pub position: PatchPosition,
    pub color: [f32; 4],
    pub glyph: char,
    pub id: u64,
}

impl Patch {
    pub fn new(id: u64, left: f32, right: f32, bottom: f32, top: f32, near: f32, color: [f32; 4], sigil: Sigil) -> Self {
        Patch {
            id: id,
            position: PatchPosition { left: left, right: right, bottom: bottom, top: top, near: near },
            color: color,
            glyph: sigil.ascii_point(),
        }
    }

    pub fn new_in_cage(cage: &Cage, color: [f32; 4], sigil: Sigil, id: u64) -> Self {
        let patch_width = if sigil.is_fill() {
            cage.frame.w
        } else {
            let width_per_height = sigil.width_per_height();
            cage.frame.h * width_per_height
        };
        let non_patch_width = cage.frame.w - patch_width;
        let patch_cage = cage.translate_sides(cage::Translation { right: -non_patch_width, ..Default::default() });
        Patch {
            id: id, glyph: sigil.ascii_point(), color: color, position: PatchPosition::from_cage(&patch_cage)
        }
    }
}
