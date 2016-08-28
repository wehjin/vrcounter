extern crate glium;

use mat;
use cam;
use std::f32::consts::PI as PI;
use glium::{Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

fn make_shape() -> Vec<Vertex> {
    let vertex1 = Vertex { position: [-0.5, -0.25, 0.0], normal: [0.0, 0.0, -1.0] };
    let vertex2 = Vertex { position: [0.0, 0.25, 0.0], normal: [0.0, 0.0, -1.0] };
    let vertex3 = Vertex { position: [0.5, -0.25, 0.0], normal: [0.0, 0.0, -1.0] };
    let shape = vec![vertex1, vertex2, vertex3];
    return shape;
}

pub struct Room {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Room {

    pub fn draw<T:Surface>(&self, surface: &mut T, view: &[[f32;4];4], projection: &[[f32;4];4]) {
        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 1.6, -1.0, 1.0f32],
        ];
        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        surface.draw(&self.vertex_buffer, &self.indices, &self.program,
                     &uniform! {model:model, view:*view, perspective:*projection},
                     &draw_params)
            .unwrap();
    }

    pub fn draw_to_camera<T:Surface>(&self, surface: &mut T, camera: &cam::Camera) {
        let view = mat::view_matrix(&camera.eye, &camera.look, &camera.up);
        let perspective = mat::perspective_matrix(surface.get_dimensions(), PI / 3.0);
        self.draw(surface, &view, &perspective);
    }

    pub fn for_display(display: &glium::Display) -> Room {
        let program = glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(display, &make_shape()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        Room { program: program, vertex_buffer: vertex_buffer, indices: indices }
    }
}


static VERTEX_SHADER: &'static str = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

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
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
