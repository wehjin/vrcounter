extern crate glium;

pub static VERTEX_SHADER: &'static str = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

pub static FRAGMENT_SHADER: &'static str = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);


pub struct Room {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices
}

impl Room {
    pub fn draw(&self, frame: &mut glium::Frame) {
        use glium::Surface;
        frame.draw(&self.vertex_buffer, &self.indices, &self.program, &glium::uniforms::EmptyUniforms,
                   &Default::default()).unwrap();
    }

    pub fn for_display(display: &glium::Display) -> Room {
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [0.0, 0.5] };
        let vertex3 = Vertex { position: [0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let program = glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let floor = Room { program: program, vertex_buffer: vertex_buffer, indices: indices };
        return floor;
    }
}

