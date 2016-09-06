#[macro_use] extern crate glium;
extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;
extern crate image;
extern crate rusttype;
extern crate unicode_normalization;
extern crate cage;

mod keymap;
mod patch_program;
mod floor_program;
mod mist_program;
mod programs;
mod mat;
mod cam;
mod user;
mod eyebuffers;
mod common;
mod os;
mod shape;
mod atlas;
mod viewer;
mod color;
mod scream;
mod howl;
mod mist;
mod patch;
mod app;
mod vision;
mod summoner;
mod roar;
mod beat;
mod eye;
mod hmd;
mod constants;
mod poses;

use openvr::Eye;
use openvr::common::{TextureBounds};
use nalgebra::{Inverse, Transpose};
use glium::{DisplayBuild};
use glium::framebuffer::{SimpleFrameBuffer, ToColorAttachment, ToDepthAttachment};
use glium::glutin::{Event, ElementState, WindowBuilder};
use programs::Programs;
use viewer::ActiveViewer;
use common::{Error, RenderSize, nmatrix4_from_steam34, raw4_from_nmatrix4};
use std::rc::Rc;
use app::{Message as AppMessage};
use std::sync::mpsc::{Sender};
use std::time::{Instant, Duration};
use hmd::Hmd;
use std::borrow::Borrow;
use poses::Poses;

pub fn main() {
    let viewer = ActiveViewer::start();
    let app = app::start(viewer.clone());
    if os::is_windows() {
        run_in_vr(viewer.clone(), app.clone());
    } else {
        user::run(viewer.clone(), app.clone());
    }
    app::stop(app);
    viewer.stop();
}

fn run_in_vr(viewer: ActiveViewer, app: Sender<AppMessage>) {
    let vr_option = System::up().ok();
    if vr_option.is_none() {
        return;
    }

    let vr: System = vr_option.unwrap();

    let can_render = vr.get_can_render();
    println!("Can render {}", can_render);

    let window = WindowBuilder::new()
        .with_title("vrcounter").with_depth_buffer(24).build_glium()
        .unwrap();

    let hmd = Hmd::new(&window, &vr);
    let (mut left_frame, mut right_frame) = (
        SimpleFrameBuffer::with_depth_buffer(&window, hmd.left_eye.buffers.color.to_color_attachment(),
                                             hmd.left_eye.buffers.depth.to_depth_attachment()).unwrap(),
        SimpleFrameBuffer::with_depth_buffer(&window, hmd.right_eye.buffers.color.to_color_attachment(),
                                             hmd.right_eye.buffers.depth.to_depth_attachment()).unwrap()
    );

    let display = Rc::new(window);
    let programs = Programs::init(display.clone(), viewer);

    let mut frame_instant = Instant::now();
    let frame_duration = Duration::from_millis(300);

    'render: loop {
        let poses = vr.await_poses();
        let world_to_hmd = poses.get_world_to_hmd_matrix();

        hmd.draw(&programs, &world_to_hmd, display.borrow(), &mut left_frame, &mut right_frame);

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => break 'render,
                Event::KeyboardInput(ElementState::Pressed, 1, _) => break 'render,
                _ => ()
            }
        }
        if Instant::now().duration_since(frame_instant) > frame_duration {
            frame_instant = Instant::now();
            app.send(AppMessage::Frame).unwrap_or(());
        }
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

fn nmatrix4_from_steam44(r: &[[f32; 4]; 4]) -> nalgebra::Matrix4<f32> {
    nalgebra::Matrix4::new(
        r[0][0], r[1][0], r[2][0], r[3][0],
        r[0][1], r[1][1], r[2][1], r[3][1],
        r[0][2], r[1][2], r[2][2], r[3][2],
        r[0][3], r[1][3], r[2][3], r[3][3],
    ).transpose()
}
