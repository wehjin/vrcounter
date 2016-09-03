#[derive(Debug, Copy, Clone)]
pub struct PatchPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32
}

#[derive(Debug, Copy, Clone)]
pub struct Patch {
    pub position: PatchPosition,
    pub color: [f32; 4],
    pub glyph: char,
    pub id: u64,
}

impl Patch {
    pub fn of_color(position: &PatchPosition, color: &[f32; 4], id: u64) -> Self {
        Patch {
            position: position.clone(),
            color: color.clone(),
            glyph: '\u{0}',
            id: id
        }
    }
}
