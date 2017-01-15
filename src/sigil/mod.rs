use cage::Cage;
use glyffin::Glyffiary;

#[derive(Copy, Clone, Debug)]
pub struct Stroke {
    cage: Cage,
    ascii_point: char,
}

#[derive(Clone, Debug)]
pub struct Sigil {
    cage: Cage,
    strokes: Vec<Stroke>
}

impl Sigil {
    pub fn of_fill() -> Self {
        let cage = Cage::from((0.0, 1.0, 0.0, 1.0, 0.0, 0.0));
        let strokes = Vec::new();
        Sigil { cage: cage, strokes: strokes }
    }

    pub fn of_point(ascii_point: char, glyffiary: &Glyffiary) -> Self {
        let stroke_width = glyffiary.advance_for_ascent(ascii_point, 1.0);
        let stroke_cage = Cage::from((0.0, stroke_width, 0.0, 1.0, 0.0, 0.0));
        let stroke = Stroke { cage: stroke_cage, ascii_point: ascii_point };
        Sigil { cage: stroke_cage, strokes: vec!(stroke) }
    }
}
