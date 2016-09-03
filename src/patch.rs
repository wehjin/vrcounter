#[derive(Debug, Copy, Clone)]
pub struct PatchPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32
}

pub enum Sigil {
    Fill,
    Letter(char),
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
            glyph: match sigil {
                Sigil::Fill => '\u{0}',
                Sigil::Letter(c) => c,
            },
        }
    }
    pub fn of_color(position: &PatchPosition, color: &[f32; 4], id: u64) -> Self {
        Patch {
            position: position.clone(),
            color: color.clone(),
            glyph: '\u{0}',
            id: id
        }
    }
}
