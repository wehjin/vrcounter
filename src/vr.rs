extern crate openvr;
extern crate openvr_sys;

use openvr::Eye;
use openvr::common::{TextureBounds};
use common::{Error, RenderSize, nmatrix4_from_steam34, raw4_from_nmatrix4, nmatrix4_from_steam44};
use poses::Poses;
use nalgebra::{Inverse};

pub struct System {
    system: openvr::system::IVRSystem,
    compositor: openvr::compositor::IVRCompositor,
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

    pub fn get_left_projection(&self) -> [[f32; 4]; 4] {
        self.get_projection(Eye::Left)
    }
    pub fn get_right_projection(&self) -> [[f32; 4]; 4] {
        self.get_projection(Eye::Right)
    }
    fn get_projection(&self, eye: Eye) -> [[f32; 4]; 4] {
        let raw_projection = self.system.projection_matrix(eye, 0.01, 1000.0);
        let nalg_projection = nmatrix4_from_steam44(&raw_projection);
        let raw_eye_to_head = self.system.eye_to_head_transform(eye);
        let nalg_eye_to_head = nmatrix4_from_steam34(&raw_eye_to_head);
        let nalg_head_to_eye = nalg_eye_to_head.inverse().unwrap();
        let nalg_combined = nalg_projection * nalg_head_to_eye;
        raw4_from_nmatrix4(&nalg_combined)
    }

    pub fn submit_textures(&self, left_texture_id: usize, right_texture_id: usize) {
        self.submit_texture(Eye::Left, left_texture_id);
        self.submit_texture(Eye::Right, right_texture_id);
    }

    fn submit_texture(&self, eye: Eye, texture_id: usize) {
        self.compositor.submit(eye, texture_id, TextureBounds::new((0.0, 1.0), (0.0, 1.0)));
    }
}

impl Drop for System {
    fn drop(&mut self) {
        openvr::shutdown();
    }
}
