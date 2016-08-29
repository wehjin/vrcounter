extern crate rusttype;
extern crate glium;

use glium::{Display};
use glium::texture::{Texture2d, RawImage2d};
use std::borrow::Cow;
use rusttype::{
    FontCollection, Font, Scale, VMetrics, CodepointOrGlyphId,
    Glyph, ScaledGlyph, HMetrics, PositionedGlyph, point, Rect
};

pub struct PagePosition {
    pub left: f32,
    pub right: f32,
}

pub struct Atlas {
    pub texture: Texture2d,
    pub page_position: PagePosition,
}

impl Atlas {
    pub fn new(display: &Display) -> Self {
        let font_data = include_bytes!("Arial Unicode.ttf");
        let font: Font = FontCollection::from_bytes(font_data as &[u8]).into_font().unwrap();
        let glyph_count = 96;
        let max_pixels_side = 512; // power of 2
        let max_pixels_total = max_pixels_side * max_pixels_side;
        let page_height = 64; // power of 2
        let average_page_width = (max_pixels_total / page_height / glyph_count) as usize;
        let (atlas_width, atlas_height) = (max_pixels_total / page_height, page_height);
        let mut atlas_data = vec![0u8; atlas_width as usize * atlas_height as usize];
        let scale = Scale::uniform(page_height as f32);
        let v_metrics = font.v_metrics(scale);
        println!("V metrics: {:?}", v_metrics);
        let font_height = v_metrics.ascent - v_metrics.descent;
        let code_point = CodepointOrGlyphId::from('J');
        let glyph: Glyph = font.glyph(code_point).unwrap();
        let scaled_glyph = glyph.scaled(scale);
        let h_metrics = scaled_glyph.h_metrics();
        println!("H metrics: {:?}", h_metrics);
        let caret = point(0.0, v_metrics.ascent);
        let positioned_glyph = scaled_glyph.positioned(caret);
        let pixel_bounding_box: Rect<i32> = positioned_glyph.pixel_bounding_box().unwrap();
        println!("Pixel bounding: {:?}", pixel_bounding_box);
        positioned_glyph.draw(|x, y, coverage| {
            let data_y = y + pixel_bounding_box.min.y as u32;
            let data_x = x + pixel_bounding_box.min.x as u32;
            let data_i = (data_y * atlas_width + data_x) as usize;
            atlas_data[data_i] = (255.0 * coverage) as u8;
        });
        let actual_page_width = pixel_bounding_box.max.x - pixel_bounding_box.min.x;
        let atlas_width_divisor = atlas_width as f32;
        let page_position = PagePosition {
            left: caret.x / atlas_width_divisor,
            right: (caret.x + h_metrics.advance_width as f32) / atlas_width_divisor,
        };
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
        Atlas { texture: atlas_texture, page_position: page_position }
    }
}
