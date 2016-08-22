extern crate glium;

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

static PI: f32 = 3.141592;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

pub struct Room {
    program: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Room {
    pub fn draw(&self, frame: &mut glium::Frame) {
        use glium::Surface;

        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];
        let view = view_matrix(&[0.0, 0.0, 0.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);
        let perspective = perspective_matrix(frame.get_dimensions(), PI / 3.0);
        let draw_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        frame.draw(&self.vertex_buffer, &self.indices, &self.program,
                   &uniform! {model:model, view:view, perspective:perspective},
                   &draw_params)
            .unwrap();
    }

    fn make_shape() -> Vec<Vertex> {
        let vertex1 = Vertex { position: [-0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0] };
        let vertex2 = Vertex { position: [0.0, 0.5, 1.0], normal: [0.0, 0.0, -1.0] };
        let vertex3 = Vertex { position: [0.5, -0.25, 1.0], normal: [0.0, 0.0, -1.0] };
        let shape = vec![vertex1, vertex2, vertex3];
        return shape;
    }

    pub fn for_display(display: &glium::Display) -> Room {
        let shape = Room::make_shape();
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let program = glium::Program::from_source(display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let floor = Room { program: program, vertex_buffer: vertex_buffer, indices: indices };
        return floor;
    }
}

fn perspective_matrix((width, height): (u32, u32), fov: f32) -> [[f32; 4]; 4] {
    let aspect_ratio = height as f32 / width as f32;
    let zfar = 1024.0;
    let znear = 0.1;
    let f = 1.0 / (fov / 2.0).tan();
    return [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}


fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
