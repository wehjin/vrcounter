extern crate glium;

use mat;
use cam;
use std::f32::consts::PI;
use glium::{Surface, VertexBuffer, Program, Display};
use glium::index::{NoIndices, PrimitiveType};
use shape::{Shape, ShapeList};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
}
implement_vertex!(Vertex, position, normal, color);

fn get_vertices_for_shape_list(shape_list: &ShapeList) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for shape in shape_list.shapes.iter() {
        let mut shape_vertices = get_vertices_for_shape(shape);
        vertices.append(&mut shape_vertices);
    }
    vertices
}

fn get_vertex_for_shape(shape: &Shape, position: [f32; 3]) -> Vertex {
    Vertex { position: position, normal: shape.normal, color: shape.color }
}

fn get_vertices_for_shape(shape: &Shape) -> Vec<Vertex> {
    let bottom_left = get_vertex_for_shape(shape, [shape.left, shape.bottom, shape.near]);
    let bottom_right = get_vertex_for_shape(shape, [shape.right, shape.bottom, shape.near]);
    let top_left = get_vertex_for_shape(shape, [shape.left, shape.top, shape.near]);
    let top_right = get_vertex_for_shape(shape, [shape.right, shape.top, shape.near]);
    vec![bottom_left, top_left, top_right, bottom_left, top_right, bottom_right]
}

pub struct PatchProgram {
    program: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    model_matrix: [[f32; 4]; 4],
}

impl PatchProgram {
    pub fn new(display: &Display, shape_list: ShapeList) -> Self {
        PatchProgram {
            program: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
            vertex_buffer: VertexBuffer::new(display, &get_vertices_for_shape_list(&shape_list)).unwrap(),
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
                     &uniform! {model: self.model_matrix, view: * view, perspective: * projection},
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
        in vec4 color;

        out vec4 vColor;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            vColor = color;
        }
    "#;

static FRAGMENT_SHADER: &'static str = r#"
        #version 140

        in vec4 vColor;

        out vec4 color;

        void main() {
            color = vColor;
        }
    "#;
