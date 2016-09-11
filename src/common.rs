extern crate openvr;
extern crate nalgebra;

#[derive(Debug)]
pub enum Error {
    NoSystem,
    NoCompositor,
}

pub enum Report<Mod, Out> {
    Unchanged,
    Model(Mod),
    Outcome(Out),
    Error,
}

#[derive(Debug)]
pub struct RenderSize {
    pub width: u32,
    pub height: u32
}

impl From<openvr::common::Size> for RenderSize {
    fn from(size: openvr::common::Size) -> Self {
        RenderSize { width: size.width, height: size.height }
    }
}

#[derive(Debug)]
pub struct IdSource {
    global_id: u64,
}

impl IdSource {
    pub fn new() -> Self {
        IdSource { global_id: 1u64 }
    }
    pub fn id(&mut self) -> u64 {
        let id = self.global_id;
        self.global_id = self.global_id + 1;
        id
    }
}

pub fn nmatrix4_from_steam34(s: &[[f32; 4]; 3]) -> nalgebra::Matrix4<f32> {
    // Both Steam and nalgebra are row-major.
    nalgebra::Matrix4::new(
        s[0][0], s[0][1], s[0][2], s[0][3],
        s[1][0], s[1][1], s[1][2], s[1][3],
        s[2][0], s[2][1], s[2][2], s[2][3],
        0.0, 0.0, 0.0, 1.0)
}

pub fn raw4_from_nmatrix4(m: &nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    // Raw is column-major.
    [
        [m.m11, m.m21, m.m31, m.m41],
        [m.m12, m.m22, m.m32, m.m42],
        [m.m13, m.m23, m.m33, m.m43],
        [m.m14, m.m24, m.m34, m.m44],
    ]
}

pub fn nmatrix4_from_steam44(s: &[[f32; 4]; 4]) -> nalgebra::Matrix4<f32> {
    // Both Steam and nalgebra are row-major.
    nalgebra::Matrix4::new(
        s[0][0], s[0][1], s[0][2], s[0][3],
        s[1][0], s[1][1], s[1][2], s[1][3],
        s[2][0], s[2][1], s[2][2], s[2][3],
        s[3][0], s[3][1], s[3][2], s[3][3],
    )
}
