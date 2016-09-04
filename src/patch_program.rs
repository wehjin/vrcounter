extern crate glium;
extern crate rusttype;
extern crate unicode_normalization;

use std::io::Cursor;
use glium::{Surface, VertexBuffer, Program, Display};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::{SrgbTexture2d, RawImage2d};
use shape::{Shape, ShapeList, ShapeMask};
use image;
use atlas::{Atlas};
use viewer::ActiveViewer;
use std::rc::Rc;
use std::borrow::Borrow;

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
        let it: &Shape = shape;
        if let ShapeMask::None = it.mask {
            let mut shape_vertices = get_vertices_for_shape(shape, atlas);
            vertices.append(&mut shape_vertices);
        }
    }
    for shape in shape_list.shapes.iter() {
        let it: &Shape = shape;
        if let ShapeMask::Letter(_) = it.mask {
            let mut shape_vertices = get_vertices_for_shape(shape, atlas);
            vertices.append(&mut shape_vertices);
        }
    }
    vertices
}

fn get_vertex_for_shape(shape: &Shape, position: [f32; 3], tex_coords: [f32; 2]) -> Vertex {
    let use_texture = match shape.mask {
        ShapeMask::None => 0.0,
        ShapeMask::Letter(_) => 1.0,
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
    let page_option = if let ShapeMask::Letter(letter) = shape.mask {
        atlas.page_map.get(&letter)
    } else {
        None
    };
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
    indices: glium::index::NoIndices,
    model_matrix: [[f32; 4]; 4],
    atlas: Atlas,
    viewer: ActiveViewer,
    display: Rc<Display>,
}

//pub fn load_galaxy(display: &Display) -> SrgbTexture2d {
//    let image = image::load(Cursor::new(&include_bytes!("galaxy.png")[..]), image::PNG).unwrap().to_rgba();
//    let image_dimensions = image.dimensions();
//    let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
//    SrgbTexture2d::new(display, image).unwrap()
//}

impl PatchProgram {
    pub fn new(display: Rc<Display>, viewer: ActiveViewer) -> Self {
        let program = {
            let display_ref: &Display = display.borrow();
            Program::from_source(display_ref, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap()
        };
        let atlas = {
            let display_ref: &Display = display.borrow();
            Atlas::new(display_ref)
        };
        PatchProgram {
            program: program,
            indices: NoIndices(PrimitiveType::TrianglesList),
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.6, -1.0, 1.0f32],
            ],
            atlas: atlas,
            viewer: viewer,
            display: display,
        }
    }

    pub fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        let uniforms = uniform! {
            model: self.model_matrix, view: ( *view), perspective: ( * projection),
            tex: self.atlas.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        let mut shape_list = ShapeList::new();
        for shape in get_shapes(&self.viewer) {
            shape_list.push(shape);
        }
        let display: &Display = self.display.borrow();
        let vertex_buffer = VertexBuffer::new(
            display,
            &get_vertices_for_shape_list(&shape_list, &self.atlas)
        ).unwrap();

        surface.draw(
            &vertex_buffer,
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

fn get_shapes(viewer: &ActiveViewer) -> Vec<Shape> {
    let patch_map = viewer.get_patch_report();
    let mut shapes = Vec::new();
    for (_, patch) in patch_map {
        let mask = if patch.glyph == '\u{0}' { ShapeMask::None } else { ShapeMask::Letter(patch.glyph) };
        let shape = Shape::new(
            patch.position.left, patch.position.right,
            patch.position.top, patch.position.bottom,
            patch.position.near, patch.color,
            patch.id, mask
        );
        shapes.push(shape);
    }
    shapes
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
