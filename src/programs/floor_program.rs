extern crate glium;

use mat::IDENTITY44;
use glium::{Display, Program, VertexBuffer, Surface};
use glium::index::{NoIndices, PrimitiveType};
use std::rc::Rc;
use std::borrow::Borrow;

pub struct FloorProgram {
    program: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    model_matrix: [[f32; 4]; 4],
}

impl FloorProgram {
    pub fn new(display: Rc<Display>) -> Self {
        let nw = Vertex { position: [-1.0, 0.0, -1.0] };
        let ne = Vertex { position: [1.0, 0.0, -1.0] };
        let se = Vertex { position: [1.0, 0.0, 1.0] };
        let sw = Vertex { position: [-1.0, 0.0, 1.0] };
        let vertices = vec![nw, ne.clone(), sw.clone(), ne, se, sw];
        let display_ref: &Display = display.borrow();
        FloorProgram {
            program: Program::from_source(display_ref, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
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

static VERTEX_SHADER: &'static str = r#"
        #version 140

        in vec3 position;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

static FRAGMENT_SHADER: &'static str = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.27, 0.33, 0.40, 1.0);
        }
    "#;
