use cage::Cage;
use glyffin::Glyffiary;
use std::str;

#[derive(Copy, Clone, Debug)]
pub struct Stroke {
    cage: Cage,
    ascii_point: char,
}

impl Stroke {
    fn new(ascii_point: char, glyffiary: &Glyffiary, left: f32) -> Self {
        let stroke_width = glyffiary.advance_for_ascent(ascii_point, 1.0);
        let stroke_cage = Cage::from((left, left + stroke_width, 0.0, 1.0, 0.0, 0.0));
        Stroke { cage: stroke_cage, ascii_point: ascii_point }
    }
    pub fn ascii_point(&self) -> char {
        self.ascii_point
    }
    pub fn cage(&self) -> &Cage {
        &self.cage
    }
}

#[derive(Clone, Debug)]
pub struct Sigil {
    cage: Cage,
    pub strokes: Vec<Stroke>
}

impl Sigil {
    pub fn of_fill() -> Self {
        let cage = Cage::from((0.0, 1.0, 0.0, 1.0, 0.0, 0.0));
        let strokes = Vec::new();
        Sigil { cage: cage, strokes: strokes }
    }

    pub fn of_line(ascii_line: &str, glyffiary: &Glyffiary) -> Self {
        let mut strokes = Vec::new();
        let mut offset: f32 = 0.0;
        let mut height: f32 = 0.0;
        for ascii_point in ascii_line.chars() {
            let stroke_left = offset;
            let stroke = Stroke::new(ascii_point, glyffiary, stroke_left);
            offset = offset + stroke.cage.frame.w;
            height = height.max(stroke.cage.frame.h);
            strokes.push(stroke)
        }
        let cage = Cage::from((0.0, offset, 0.0, height, 0.0, 0.0));
        Sigil { cage: cage, strokes: strokes }
    }

    pub fn of_point(ascii_point: char, glyffiary: &Glyffiary) -> Self {
        let stroke = Stroke::new(ascii_point, glyffiary, 0.0);
        let cage = stroke.cage;
        Sigil { cage: cage, strokes: vec![stroke] }
    }

    pub fn is_fill(&self) -> bool {
        self.strokes.is_empty()
    }

    pub fn width_per_height(&self) -> f32 {
        if self.is_fill() {
            panic!("width_per_height in not supported for sigils of fill")
        } else {
            self.strokes[0].cage.frame.w
        }
    }

    pub fn ascii_point(&self) -> char {
        if self.is_fill() {
            '\u{0}'
        } else {
            self.strokes[0].ascii_point
        }
    }
}
