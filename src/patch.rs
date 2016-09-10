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

#[derive(Copy, Clone)]
pub enum Sigil {
    Fill,
    Letter(char),
}

impl Sigil {
    pub fn to_glyph(&self) -> char {
        match self {
            &Sigil::Fill => '\u{0}',
            &Sigil::Letter(c) => c,
        }
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
            glyph: sigil.to_glyph(),
        }
    }
    pub fn from_cage(cage: &Cage, color: [f32; 4], sigil: Sigil, id: u64) -> Self {
        Patch {
            id: id, glyph: sigil.to_glyph(), color: color, position: PatchPosition::from_cage(cage)
        }
    }
}
