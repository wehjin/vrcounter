extern crate glium;

use glium::{Surface, VertexBuffer, Program, Display};
use glium::index::{NoIndices, PrimitiveType};
use std::rc::Rc;
use std::borrow::Borrow;
use viewer::Viewer;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub struct HandProgram {
    display: Rc<Display>,
    program: glium::Program,
    indices: glium::index::NoIndices,
    viewer: Viewer,
    model_matrix: [[f32; 4]; 4],
}

impl HandProgram {
    pub fn new(display: Rc<Display>, viewer: Viewer) -> Self {
        HandProgram {
            display: display.clone(),
            program: Program::from_source(display.borrow() as &Display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
            indices: NoIndices(PrimitiveType::LinesList),
            viewer: viewer,
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.5, -0.95, 1.0f32],
            ],
        }
    }

    fn get_vertex_buffer(&self) -> VertexBuffer<Vertex> {
        const RADIUS: f32 = 0.05;
        let hand = self.viewer.get_hand();
        let center = hand.offset;
        let (l, r, b, t) = (center.x - RADIUS, center.x + RADIUS, center.y - RADIUS, center.y + RADIUS);
        let left = Vertex { position: [l, center.y, center.z] };
        let right = Vertex { position: [r, center.y, center.z] };
        let bottom = Vertex { position: [center.x, b, center.z] };
        let top = Vertex { position: [center.x, t, center.z] };
        let mut vertices = Vec::new();
        vertices.push(left);
        vertices.push(right);
        vertices.push(bottom);
        vertices.push(top);
        VertexBuffer::new(self.display.borrow() as &Display, &vertices).unwrap()
    }

    pub fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        let vertex_buffer = self.get_vertex_buffer();
        let uniforms = uniform! { model: self.model_matrix, view: * view, perspective: * projection };
        surface.draw(
            &vertex_buffer,
            &self.indices,
            &self.program,
            &uniforms,
            &glium::DrawParameters {
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
            color = vec4(0.8, 1.0, 0.8, 0.8);
        }
    "#;
