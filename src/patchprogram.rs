extern crate glium;

use mat;
use cam;
use std::f32::consts::PI;
use glium::{Surface, VertexBuffer, Program};
use glium::index::{NoIndices, PrimitiveType};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

struct Shape {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32,
    normal: [f32; 3],
}

impl Shape {
    fn new() -> Self {
        Shape { left: -0.5, right: 0.5, top: 0.25, bottom: -0.25, near: 0.0, normal: [0.0, 0.0, -1.0] }
    }

    fn to_vertices(&self) -> Vec<Vertex> {
        let bottom_left = Vertex { position: [self.left, self.bottom, self.near], normal: self.normal };
        let bottom_right = Vertex { position: [self.right, self.bottom, self.near], normal: self.normal };
        let top_left = Vertex { position: [self.left, self.top, self.near], normal: self.normal };
        let top_right = Vertex { position: [self.right, self.top, self.near], normal: self.normal };
        vec![bottom_left, top_left, top_right, bottom_left, top_right, bottom_right]
    }
}

pub struct PatchProgram {
    program: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    model_matrix: [[f32; 4]; 4],
}

impl PatchProgram {
    pub fn new(display: &glium::Display) -> Self {
        let shape = Shape::new();
        PatchProgram {
            program: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
            vertex_buffer: VertexBuffer::new(display, &shape.to_vertices()).unwrap(),
            indices: NoIndices(PrimitiveType::TrianglesList),
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.6, -1.0, 1.0f32],
            ],
        }
    }

    pub fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        surface.draw(&self.vertex_buffer, &self.indices, &self.program,
                     &uniform! {model:self.model_matrix, view:*view, perspective:*projection},
                     &draw_params)
            .unwrap();
    }

    pub fn draw_to_camera<T: Surface>(&self, surface: &mut T, camera: &cam::Camera) {
        let view = mat::view_matrix(&camera.eye, &camera.look, &camera.up);
        let perspective = mat::perspective_matrix(surface.get_dimensions(), PI / 3.0);
        self.draw(surface, &view, &perspective);
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
