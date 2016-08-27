#[macro_use] extern crate glium;
extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;

pub mod world;
pub mod mat;
pub mod cam;
pub mod app;

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
}

impl Drop for System {
    fn drop(&mut self) {
        openvr::shutdown();
    }
}
