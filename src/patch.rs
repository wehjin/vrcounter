extern crate cage;

use cage::Cage;

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

pub const FILL_POINT: char = '\u{0}';

#[derive(Debug, Copy, Clone)]
pub struct Patch {
    pub position: PatchPosition,
    pub color: [f32; 4],
    pub glyph: char,
    pub id: u64,
}

impl Patch {
    pub fn new(id: u64, left: f32, right: f32, bottom: f32, top: f32, near: f32, color: [f32; 4], ascii_point: char) -> Self {
        Patch {
            id: id,
            position: PatchPosition { left: left, right: right, bottom: bottom, top: top, near: near },
            color: color,
            glyph: ascii_point,
        }
    }
}
