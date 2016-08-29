extern crate glium;
extern crate rusttype;
extern crate unicode_normalization;

use mat;
use cam;
use std::f32::consts::PI;
use std::io::Cursor;
use glium::{Surface, VertexBuffer, Program, Display};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{SrgbTexture2d, RawImage2d};
use shape::{Shape, ShapeList, ShapeMask};
use image;
use atlas::{Atlas, AtlasPage};


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
    tex_coords: [f32; 2],
    use_texture: f32,
}
implement_vertex!(Vertex, position, normal, color, tex_coords, use_texture);

fn get_vertices_for_shape_list(shape_list: &ShapeList, atlas: &Atlas) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for shape in shape_list.shapes.iter() {
        let mut shape_vertices = get_vertices_for_shape(shape, atlas);
        vertices.append(&mut shape_vertices);
    }
    vertices
}

fn get_vertex_for_shape(shape: &Shape, position: [f32; 3], tex_coords: [f32; 2]) -> Vertex {
    let use_texture = match shape.mask {
        ShapeMask::None => 0.0,
        ShapeMask::Zero => 1.0,
    };
    Vertex {
        position: position,
        normal: shape.normal,
        color: shape.color,
        tex_coords: tex_coords,
        use_texture: use_texture
    }
}

fn get_vertices_for_shape(shape: &Shape, atlas: &Atlas) -> Vec<Vertex> {
    let page_option = atlas.page_map.get(&'E');
    let (texture_left, texture_right) = match page_option {
        None => (0.0, 0.0),
        Some(page) => (page.left, page.right),
    };
    let bottom_left = get_vertex_for_shape(shape, [shape.left, shape.bottom, shape.near], [texture_left, 1.0]);
    let bottom_right = get_vertex_for_shape(shape, [shape.right, shape.bottom, shape.near], [texture_right, 1.0]);
    let top_left = get_vertex_for_shape(shape, [shape.left, shape.top, shape.near], [texture_left, 0.0]);
    let top_right = get_vertex_for_shape(shape, [shape.right, shape.top, shape.near], [texture_right, 0.0]);
    vec![bottom_left, top_left, top_right, bottom_left, top_right, bottom_right]
}

pub struct PatchProgram {
    program: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
    model_matrix: [[f32; 4]; 4],
    atlas: Atlas
}

fn load_galaxy(display: &Display) -> SrgbTexture2d {
    let image = image::load(Cursor::new(&include_bytes!("galaxy.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    SrgbTexture2d::new(display, image).unwrap()
}

impl PatchProgram {
    pub fn new(display: &Display, shape_list: ShapeList) -> Self {
        let atlas = Atlas::new(display);
        PatchProgram {
            program: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
            vertex_buffer: VertexBuffer::new(
                display,
                &get_vertices_for_shape_list(&shape_list, &atlas)
            ).unwrap(),
            indices: NoIndices(PrimitiveType::TrianglesList),
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.6, -1.0, 1.0f32],
            ],
            atlas: atlas,
        }
    }

    pub fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        let uniforms = uniform! {
            model: self.model_matrix, view: ( *view), perspective: ( * projection),
            tex: self.atlas.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
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
        in vec2 tex_coords;
        in float use_texture;

        out vec4 vColor;
        out vec2 vTexCoords;
        out float vUseTexture;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            vColor = color;
            vTexCoords = tex_coords;
            vUseTexture = use_texture;
        }
    "#;

static FRAGMENT_SHADER: &'static str = r#"
        #version 140

        in vec4 vColor;
        in vec2 vTexCoords;
        in float vUseTexture;

        out vec4 color;

        uniform sampler2D tex;

        void main() {
            if (vUseTexture > 0.5) {
                vec4 tColor = texture(tex, vTexCoords);
                color = vColor * vec4(1.0, 1.0, 1.0, tColor.r);
            } else {
                color = vColor;
            }
        }
    "#;
