extern crate glium;
extern crate openvr;
extern crate openvr_sys;

use mat::IDENTITY44;
use glium::{Display, Program, VertexBuffer, IndexBuffer, Surface};
use glium::index::{PrimitiveType};
use glium::texture::{RawImage2d, Texture2d};
use openvr::render_models::{IVRRenderModels, RenderModel, RenderModelTexture};
use std::ffi::CString;

pub struct ControllerProgram {
    program: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    model_matrix_option: Option<[[f32; 4]; 4]>,
    texture: Texture2d,
}

pub fn get_name(render_models: &IVRRenderModels, index: u32) -> String {
    unsafe {
        let models = *{ render_models.0 as *mut openvr_sys::VR_IVRRenderModels_FnTable };
        let name_function = models.GetRenderModelName.unwrap();
        let mut empty = vec![0i8;0];
        let required = name_function(index, empty.as_mut_ptr(), 0);
        if required == 0 {
            return String::from("")
        }
        let mut data: Vec<u8> = Vec::with_capacity(required as usize);
        let size = name_function(index, data.as_mut_ptr() as *mut i8, required);
        if (size != required) {
            panic!("name size changed");
        }
        let size_without_null = size - 1;
        data.set_len(size_without_null as usize);
        if let Ok(string) = CString::from_vec_unchecked(data).into_string() {
            string
        } else {
            panic!("name cannot convert to utf8");
        }
    }
}

impl ControllerProgram {
    pub fn new(display: &Display) -> Self {
        let render_models: IVRRenderModels = openvr::subsystems::render_models().unwrap();
        let count = render_models.get_count();
        println!("Render model names: {:?}", count);
        for index in 0..count {
            let name = get_name(&render_models, index);
            println!("{} {}", index + 1, name);
        }
        let render_model: RenderModel = render_models.load(String::from("vr_controller_vive_1_5")).unwrap();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        for vertex in render_model.vertex_iter() {
            vertices.push(Vertex {
                position: [vertex.vPosition.v[0] as f32, vertex.vPosition.v[1] as f32, vertex.vPosition.v[2] as f32],
                normal: [vertex.vNormal.v[0] as f32, vertex.vNormal.v[1] as f32, vertex.vNormal.v[2] as f32],
                texcoord: [vertex.rfTextureCoord[0] as f32, vertex.rfTextureCoord[1] as f32],
            });
        }
        for index in render_model.index_iter() {
            indices.push(*index);
        }
        let stream_texture: RenderModelTexture = render_model.load_texture().unwrap();
        let dimension = (stream_texture.dimension().0 as u32, stream_texture.dimension().1 as u32);
        let image = RawImage2d::from_raw_rgba(stream_texture.to_vec(), dimension);
        let glium_texture: Texture2d = Texture2d::new(display, image).unwrap();
        ControllerProgram {
            program: Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
            vertex_buffer: VertexBuffer::new(display, &vertices).unwrap(),
            index_buffer: IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).unwrap(),
            model_matrix_option: Option::None,
            texture: glium_texture,
        }
    }

    pub fn set_model_matrix(&mut self, model_matrix_option: &Option<[[f32; 4]; 4]>) {
        self.model_matrix_option = *model_matrix_option;
    }

    pub fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        if let Some(model_matrix) = self.model_matrix_option {
            let uniforms = uniform! { model: model_matrix, view: *view, perspective: *projection, diffuse: &self.texture };
            surface.draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &glium::DrawParameters {
                    blend: glium::Blend::default(),
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                    ..Default::default()
                }
            ).unwrap();
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texcoord: [f32; 2],
}
implement_vertex!(Vertex, position, normal, texcoord);

static VERTEX_SHADER: &'static str = r#"
        #version 140

        in vec3 position;
        in vec3 normal;
        in vec2 texcoord;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        out vec3 v_normal;
        out vec2 v_texcoord;

        void main() {
            v_normal = normal;
            v_texcoord = texcoord;
            mat4 modelview = view * model;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

static FRAGMENT_SHADER: &'static str = r#"
        #version 140

        uniform sampler2D diffuse;
        in vec3 v_normal;
        in vec2 v_texcoord;
        out vec4 color;

        void main() {
            color = texture(diffuse, v_texcoord);
        }
    "#;
