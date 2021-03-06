extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;

use openvr::tracking::{TrackedDevicePose, TrackedDevicePoses, TrackedDeviceClass};
use nalgebra::{Inverse};
use common::{nmatrix4_from_steam34, raw4_from_nmatrix4};
use openvr::tracking::TrackedDeviceStringProperty;

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
    pub fn get_world_to_hmd_matrix(&self) -> [[f32; 4]; 4] {
        let hmd: &TrackedDevicePose = self.get_hmd_pose();
        let raw_hmd_to_world = hmd.to_device;
        let nalg_hmd_to_world = nmatrix4_from_steam34(&raw_hmd_to_world);
        let nalg_world_to_hmd = nalg_hmd_to_world.inverse().unwrap();
        raw4_from_nmatrix4(&nalg_world_to_hmd)
    }

    pub fn get_controller_to_world_matrix(&self) -> Option<[[f32; 4]; 4]> {
        if let Some(ref controller) = self.get_controller_pose() {
            let s = controller.to_device;
            Some([
                [s[0][0], s[1][0], s[2][0], 0.0],
                [s[0][1], s[1][1], s[2][1], 0.0],
                [s[0][2], s[1][2], s[2][2], 0.0],
                [s[0][3], s[1][3], s[2][3], 1.0f32],
            ])
        } else {
            None
        }
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
            let render_model_name = pose.get_property_string(TrackedDeviceStringProperty::RenderModelName).unwrap();
            println!("Class:{:?}, valid:{}, connected:{}, rm-name:{}, {:?}", pose.device_class(), pose.is_valid, pose.is_connected, render_model_name, pose);
        }
    }

    fn get_controller_pose(&self) -> Option<&TrackedDevicePose> {
        self.poses.poses.iter()
                        .filter(|&x| match x.device_class() {
                            TrackedDeviceClass::Controller => x.is_valid && x.is_connected,
                            _ => false
                        })
                        .last()
    }

    fn get_hmd_pose(&self) -> &TrackedDevicePose {
        self.poses.poses.iter()
                        .filter(|&x| match x.device_class() {
                            TrackedDeviceClass::HMD => true,
                            _ => false
                        })
                        .last().unwrap()
    }
}

