extern crate rusttype;
extern crate glium;

use glium::{Display};
use glium::texture::{Texture2d, RawImage2d};
use std::borrow::Cow;
use std::collections::HashMap;
use std::char;
use rusttype::{
    FontCollection, Font, Scale, VMetrics, CodepointOrGlyphId,
    Glyph, ScaledGlyph, HMetrics, PositionedGlyph, point, Rect
};

pub struct AtlasPage {
    pub left: f32,
    pub right: f32,
}

pub struct Atlas {
    pub texture: Texture2d,
    pub page_map: HashMap<char, AtlasPage>
}

impl Atlas {
    pub fn new(display: &Display) -> Self {
        let mut page_map = HashMap::new();

        let font_data = include_bytes!("Arial Unicode.ttf");
        let font: Font = FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap();
        let glyph_count = 96;
        let max_pixels_side = 512; // power of 2
        let max_pixels_total = max_pixels_side * max_pixels_side;
        let page_height = 64; // power of 2
        let (atlas_width, atlas_height) = (max_pixels_total / page_height, page_height);
        let mut atlas_data = vec![0u8; atlas_width as usize * atlas_height as usize];
        let scale = Scale::uniform(page_height as f32);

        let v_metrics: VMetrics = font.v_metrics(scale);
        println!("V metrics: {:?}", v_metrics);
        let mut caret = point(0.0, v_metrics.ascent);
        let average_page_width = (atlas_width / glyph_count) as usize;
        for i in 32u32..128u32 {
            let i_char = if let Some(c) = char::from_u32(i) {
                c
            } else {
                continue;
            };
            let code_point = CodepointOrGlyphId::from(i_char);
            let glyph: Glyph = font.glyph(code_point).unwrap();
            let scaled_glyph: ScaledGlyph = glyph.scaled(scale);
            let h_metrics: HMetrics = scaled_glyph.h_metrics();
            println!("H metrics: {:?}", h_metrics);
            let page_right = caret.x + h_metrics.advance_width;
            if page_right >= atlas_width as f32 {
                break;
            } else {
                let positioned_glyph: PositionedGlyph = scaled_glyph.positioned(caret);
                let pixel_bounding_box: Rect<i32> = if let Some(rect) = positioned_glyph.pixel_bounding_box() {
                    rect
                } else {
                    continue;
                };
                println!("Pixel bounding: {:?}", pixel_bounding_box);
                let draw_left = pixel_bounding_box.min.x as u32;
                let draw_bottom = pixel_bounding_box.min.y as u32;
                positioned_glyph.draw(|x, y, coverage| {
                    let data_y = draw_bottom + y;
                    let data_x = draw_left + x;
                    let data_i = (data_y * atlas_width + data_x) as usize;
                    atlas_data[data_i] = (255.0 * coverage) as u8;
                });

                let atlas_page = AtlasPage {
                    left: caret.x / atlas_width as f32,
                    right: page_right / atlas_width as f32,
                };
                page_map.insert(i_char, atlas_page);
                caret.x = page_right;
            }
        }
        let atlas_texture = Texture2d::with_format(
            display,
            RawImage2d {
                data: Cow::Owned(atlas_data),
                width: atlas_width,
                height: atlas_height,
                format: glium::texture::ClientFormat::U8
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap
        ).unwrap();
        Atlas { texture: atlas_texture, page_map: page_map }
    }
}
