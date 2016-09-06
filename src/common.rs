extern crate openvr;
extern crate nalgebra;

use nalgebra::{Transpose};

#[derive(Debug)]
pub enum Error {
    NoSystem,
    NoCompositor,
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
    pub fn next_id(&mut self) -> u64 {
        let id = self.global_id;
        self.global_id = self.global_id + 1;
        id
    }
}

pub fn nmatrix4_from_steam34(r: &[[f32; 4]; 3]) -> nalgebra::Matrix4<f32> {
    nalgebra::Matrix4::new(
        r[0][0], r[1][0], r[2][0], 0.0,
        r[0][1], r[1][1], r[2][1], 0.0,
        r[0][2], r[1][2], r[2][2], 0.0,
        r[0][3], r[1][3], r[2][3], 1.0).transpose()
}

pub fn raw4_from_nmatrix4(m: &nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [m.m11, m.m21, m.m31, m.m41],
        [m.m12, m.m22, m.m32, m.m42],
        [m.m13, m.m23, m.m33, m.m43],
        [m.m14, m.m24, m.m34, m.m44],
    ]
}

pub fn nmatrix4_from_steam44(r: &[[f32; 4]; 4]) -> nalgebra::Matrix4<f32> {
    nalgebra::Matrix4::new(
        r[0][0], r[1][0], r[2][0], r[3][0],
        r[0][1], r[1][1], r[2][1], r[3][1],
        r[0][2], r[1][2], r[2][2], r[3][2],
        r[0][3], r[1][3], r[2][3], r[3][3],
    ).transpose()
}
