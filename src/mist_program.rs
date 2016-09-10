extern crate glium;

use glium::{Surface, VertexBuffer, Program, Display};
use glium::index::{NoIndices, PrimitiveType};
use cage::Cage;
use std::rc::Rc;
use std::borrow::Borrow;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub struct MistProgram {
    display: Rc<Display>,
    program: glium::Program,
    indices: glium::index::NoIndices,
    model_matrix: [[f32; 4]; 4],
    vertex_buffer_option: Option<VertexBuffer<Vertex>>,
}

impl MistProgram {
    pub fn new(display: Rc<Display>) -> Self {
        MistProgram {
            display: display.clone(),
            program: Program::from_source(display.borrow() as &Display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
            vertex_buffer_option: None,
            indices: NoIndices(PrimitiveType::LinesList),
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.6, -1.0, 1.0f32],
            ],
        }
    }

    pub fn update(&mut self, cages: Vec<Cage>) {
        self.vertex_buffer_option = if cages.is_empty() {
            None
        } else {
            let mut vertices = Vec::new();
            for cage in &cages {
                let (l, r, b, t, f, n) = cage.limits();
                let high_nw = Vertex { position: [l, t, f] };
                let high_sw = Vertex { position: [l, t, n] };
                let high_ne = Vertex { position: [r, t, f] };
                let high_se = Vertex { position: [r, t, n] };
                let low_nw = Vertex { position: [l, b, f] };
                let low_sw = Vertex { position: [l, b, n] };
                let low_ne = Vertex { position: [r, b, f] };
                let low_se = Vertex { position: [r, b, n] };
                vertices.push(high_nw);
                vertices.push(high_sw);
                vertices.push(high_ne);
                vertices.push(high_se);
                vertices.push(low_nw);
                vertices.push(low_sw);
                vertices.push(low_ne);
                vertices.push(low_se);
            }
            Some(VertexBuffer::new(self.display.borrow() as &Display, &vertices).unwrap())
        }
    }

    pub fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        if let Some(ref vertex_buffer) = self.vertex_buffer_option {
            let uniforms = uniform! { model: self.model_matrix, view: *view, perspective: * projection };
            surface.draw(
                vertex_buffer,
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
}

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
            color = vec4(0.0, 0.0, 1.0, 0.5);
        }
    "#;
