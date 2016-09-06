extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;

use openvr::tracking::{TrackedDevicePose, TrackedDevicePoses, TrackedDeviceClass};
use nalgebra::{Inverse};
use common::{nmatrix4_from_steam34, raw4_from_nmatrix4};

#[derive(Debug)]
pub struct Poses {
    poses: TrackedDevicePoses
}

impl From<TrackedDevicePoses> for Poses {
    fn from(poses: TrackedDevicePoses) -> Self {
        Poses { poses: poses }
    }
}

impl Poses {
    fn get_hmd_pose(&self) -> &TrackedDevicePose {
        self.poses.poses.iter()
                        .filter(|&x| match x.device_class() {
                            TrackedDeviceClass::HMD => true,
                            _ => false
                        })
                        .last().unwrap()
    }

    pub fn get_world_to_hmd_matrix(&self) -> [[f32; 4]; 4] {
        let hmd: &TrackedDevicePose = self.get_hmd_pose();
        let raw_hmd_to_world = hmd.to_device;
        let nalg_hmd_to_world = nmatrix4_from_steam34(&raw_hmd_to_world);
        let nalg_world_to_hmd = nalg_hmd_to_world.inverse().unwrap();
        raw4_from_nmatrix4(&nalg_world_to_hmd)
    }

    pub fn audit(&self) {
        println!("Count {}", self.poses.count);
        let poses: [TrackedDevicePose; 16] = self.poses.poses;
        let poses_iter = poses.iter().filter(
            |&x| match x.device_class() {
                openvr::tracking::TrackedDeviceClass::Invalid => false,
                _ => true
            });
        for it in poses_iter {
            let pose: &TrackedDevicePose = it;
            println!("Class:{:?}, valid:{}, connected:{}, {:?}", pose.device_class(), pose.is_valid, pose.is_connected, pose);
        }
    }
}

