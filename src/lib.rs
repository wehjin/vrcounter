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

pub struct System {
    ovr: openvr::system::IVRSystem,
}

impl System {
    pub fn up() -> Result<System, Error> {
        let ovr = try!(openvr::init().map_err(|_| Error::NoSystem));
        Ok(System { ovr: ovr })
    }

    pub fn get_render_size(&self) -> RenderSize {
        let size = self.ovr.recommended_render_target_size();
        RenderSize::from(size)
    }

    pub fn get_name(&self) -> String {
        String::from("hey")
    }
}

impl Drop for System {
    fn drop(&mut self) {
        openvr::shutdown();
    }
}
