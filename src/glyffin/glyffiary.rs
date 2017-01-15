use rusttype::{FontCollection, Font, CodepointOrGlyphId, Glyph, Scale, ScaledGlyph, HMetrics, VMetrics};
use std::collections::HashMap;
use std::char;
use glyffin::Glyffic;

pub fn load_font<'a>() -> Font<'a> {
    let font_data = include_bytes!("ubuntu-font-family-0.83/UbuntuMono-R.ttf");
    FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap()
}

#[derive(Clone, Debug)]
pub struct Glyffiary {
    glyffics: HashMap<char, Glyffic>,
}

impl Glyffiary {
    pub fn advance_for_ascent(&self, glyph_id: char, ascent: f32) -> f32 {
        self.glyffics.get(&glyph_id).map_or(0.0, |x| { ascent * x.advance_per_ascent })
    }
    pub fn new() -> Self {
        let font = load_font();
        let column_height = 64; // power of 2
        let scale = Scale::uniform(column_height as f32);
        let v_metrics: VMetrics = font.v_metrics(scale);
        let mut glyffics = HashMap::new();
        for i in 32u32..128u32 {
            let i_char = if let Some(c) = char::from_u32(i) { c } else { continue; };
            let code_point = CodepointOrGlyphId::from(i_char);
            let glyph: Glyph = font.glyph(code_point).unwrap();
            let scaled_glyph: ScaledGlyph = glyph.scaled(scale);
            let h_metrics: HMetrics = scaled_glyph.h_metrics();
            let advance_per_ascent = h_metrics.advance_width / v_metrics.ascent;
            let glyffic = Glyffic { advance_per_ascent: advance_per_ascent };
            glyffics.insert(i_char, glyffic);
        }
        Glyffiary { glyffics: glyffics }
    }
}
