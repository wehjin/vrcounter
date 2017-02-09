extern crate glium;

use glium::{Display, Program, VertexBuffer, Surface};
use glium::index::{NoIndices, PrimitiveType};
use mat::IDENTITY44;
use std::rc::Rc;
use std::borrow::Borrow;

pub struct KeyshieldProgram {
    program: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    model_matrix: [[f32; 4]; 4],
}

impl KeyshieldProgram {
    pub fn new(display: Rc<Display>) -> Self {
        let vshader = include_str!("keyshield_vshader.glsl");
        let fshader = include_str!("keyshield_fshader.glsl");
        let eye_height = 1.5;
        let crest_y = eye_height - 0.10;
        let crest_x = -0.15;
        let crest_z = -0.25;
        let crest_halfwidth = 0.035;
        let nw = Vertex { position: [crest_x + -crest_halfwidth, crest_y + -crest_halfwidth, crest_z] };
        let ne = Vertex { position: [crest_x + crest_halfwidth, crest_y + -crest_halfwidth, crest_z] };
        let se = Vertex { position: [crest_x + crest_halfwidth, crest_y + crest_halfwidth, crest_z] };
        let sw = Vertex { position: [crest_x + -crest_halfwidth, crest_y + crest_halfwidth, crest_z] };
        let vertices = vec![nw, ne.clone(), sw.clone(), ne, se, sw];
        let display_ref: &Display = display.borrow();
        KeyshieldProgram {
            program: Program::from_source(display_ref, vshader, fshader, None).unwrap(),
            vertex_buffer: VertexBuffer::new(display_ref, &vertices).unwrap(),
            indices: NoIndices(PrimitiveType::TrianglesList),
            model_matrix: IDENTITY44,
        }
    }

    pub fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        let uniforms = uniform! {
            model: self.model_matrix, view: *view, perspective: *projection,
        };
        surface.draw(
            &self.vertex_buffer,
            &self.indices,
            &self.program,
            &uniforms,
            &glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            }
        ).unwrap();
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);
