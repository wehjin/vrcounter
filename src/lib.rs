#[macro_use] extern crate glium;
extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;

pub mod world;
pub mod mat;
pub mod cam;
pub mod app;

use openvr::tracking::{TrackedDevicePoses, TrackedDevicePose};
use nalgebra::{Matrix4,Inverse, Row, Column};

#[derive(Debug)]
pub enum Error {
    NoSystem,
    NoCompositor,
}

#[derive(Debug)]
pub struct RenderSize {
    width: u32,
    height: u32
}

impl From<openvr::common::Size> for RenderSize {
    fn from(size: openvr::common::Size) -> Self {
        RenderSize { width: size.width, height: size.height }
    }
}

#[derive(Debug)]
pub struct Poses {
    poses: openvr::tracking::TrackedDevicePoses
}

impl From<openvr::tracking::TrackedDevicePoses> for Poses {
    fn from(poses: openvr::tracking::TrackedDevicePoses) -> Self {
        Poses {poses: poses}
    }
}

impl Poses {

    fn get_hmd(&self) -> &openvr::tracking::TrackedDevicePose {
        self.poses.poses.iter()
            .filter(|&x| match x.device_class() {
                openvr::tracking::TrackedDeviceClass::HMD => true,
                _ => false
            })
            .last().unwrap()
    }
    pub fn audit(&self) {
        println!("Count {}", self.poses.count);
        let poses : [TrackedDevicePose;16] = self.poses.poses;
        let poses_iter = poses.iter().filter(
            |&x| match x.device_class() {
                openvr::tracking::TrackedDeviceClass::Invalid => false,
                _ => true
            });
        for it in poses_iter {
            let pose : &openvr::tracking::TrackedDevicePose = it;
            println!("Class:{:?}, valid:{}, connected:{}, {:?}", pose.device_class(), pose.is_valid, pose.is_connected, pose);
        }
    }
}

pub struct System {
    system: openvr::system::IVRSystem,
    compositor: openvr::compositor::IVRCompositor,
}

fn raw4_from_nmatrix4(m: &nalgebra::Matrix4<f32>) -> [[f32;4];4] {
    [
        [m.m11, m.m21, m.m31, m.m41],
        [m.m12, m.m22, m.m32, m.m42],
        [m.m13, m.m23, m.m33, m.m43],
        [m.m14, m.m24, m.m34, m.m44],
    ]
}

fn nmatrix4_from_raw4(r: &[[f32;4];4]) -> nalgebra::Matrix4<f32>{
    Matrix4::new(
        r[0][0], r[1][0], r[2][0], r[3][0],
        r[0][1], r[1][1], r[2][1], r[3][1],
        r[0][2], r[1][2], r[2][2], r[3][2],
        r[0][3], r[1][3], r[2][3], r[3][3],
    )
}

impl System {
    pub fn up() -> Result<System, Error> {
        let system = try!(openvr::init().map_err(|_| Error::NoSystem));
        let compositor = try!(openvr::subsystems::compositor().map_err(|_| Error::NoCompositor));
        Ok(System { system: system, compositor: compositor })
    }

    pub fn get_render_size(&self) -> RenderSize {
        let size = self.system.recommended_render_target_size();
        RenderSize::from(size)
    }

    pub fn get_can_render(&self) -> bool {
        self.compositor.can_render_scene()
    }

    pub fn await_poses(&self) -> Poses {
        Poses::from(self.compositor.wait_get_poses())
    }

    pub fn get_left_projection(&self) -> [[f32;4];4] {
        let raw_projection = self.system.projection_matrix(openvr::Eye::Left, 0.01, 1000.0);
        let nalg_projection = nmatrix4_from_raw4(&raw_projection);
        let raw_transform = self.system.eye_to_head_transform(openvr::Eye::Left);
        let nalg_transform = nalgebra::Matrix4::new(
            raw_transform[0][0], raw_transform[1][0], raw_transform[2][0], 0.0,
            raw_transform[0][1], raw_transform[1][1], raw_transform[2][1], 0.0,
            raw_transform[0][2], raw_transform[1][2], raw_transform[2][2], 0.0,
            raw_transform[0][3], raw_transform[1][3], raw_transform[2][3], 1.0);
        let nalg_combined = nalg_projection * (nalg_transform.inverse().unwrap());
        raw4_from_nmatrix4(&nalg_combined)
    }
}

impl Drop for System {
    fn drop(&mut self) {
        openvr::shutdown();
    }
}
