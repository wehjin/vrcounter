#[macro_use] extern crate glium;
extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;
extern crate image;
extern crate rusttype;
extern crate unicode_normalization;

mod patch_program;
mod floor_program;
mod mat;
mod cam;
mod app;
mod eyebuffers;
mod common;
mod os;
mod shape;
mod atlas;
pub mod color;
pub mod scream;
pub mod shout;

use openvr::Eye;
use openvr::tracking::{TrackedDevicePose, TrackedDevicePoses, TrackedDeviceClass};
use openvr::common::{TextureBounds};
use nalgebra::{Inverse, Transpose};
use glium::{DisplayBuild, Surface, Display, GlObject};
use glium::framebuffer::{SimpleFrameBuffer, ToColorAttachment, ToDepthAttachment};
use glium::glutin::{Event, ElementState};
use std::{thread, time};
use eyebuffers::{EyeBuffers};
use common::{Error, RenderSize};
use patch_program::{PatchProgram};
use floor_program::{FloorProgram};
use shape::{Shape, ShapeList, ShapeMask};
use scream::{ScreamPosition, Viewer, IdSource};

fn get_shapes() -> Vec<Shape> {
    let mut shapes = Vec::new();
    let viewer = Viewer::start();
    let mut id_source = IdSource::new();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: -0.15, bottom: -0.25, near: 0.03 };
    let scream = scream::of_color(color::YELLOW)
        .join_right(0.1, scream::of_color(color::MAGENTA)
            .join_right(0.1, scream::of_color(color::CYAN))
        );
    scream.present(&position, &mut id_source, viewer.clone());

    let patch_map = viewer.get_report();
    for (_, patch) in patch_map {
        let shape = Shape::new(patch.position.left, patch.position.right,
                               patch.position.top, patch.position.bottom,
                               patch.position.near, patch.color,
                               patch.id, ShapeMask::Letter(patch.glyph));
        shapes.push(shape);
    }
    viewer.stop();
    shapes
}

pub fn main() {
    let mut shape_list = ShapeList::new();
    shape_list.push(Shape::new(-0.5, 0.5, 0.25, -0.25, 0.0, color::RED, 0, ShapeMask::None));
    shape_list.push(Shape::new(0.25, 0.75, 0.5, 0.0, -0.01, color::GREEN, 1, ShapeMask::None));
    shape_list.push(Shape::new(-0.06, 0.00, 0.03, -0.03, 0.005, color::CYAN, 2, ShapeMask::Letter('J')));
    shape_list.push(Shape::new(0.00, 0.06, 0.03, -0.03, 0.005, color::YELLOW, 2, ShapeMask::Letter('y')));
    let more_shapes = get_shapes();
    for shape in more_shapes {
        shape_list.push(shape);
    }
    if os::is_windows() {
        run_in_vr(shape_list)
    } else {
        run_in_nr(shape_list)
    }
}

fn run_in_nr(shape_list: ShapeList) {
    let mut model = app::Model::init(shape_list);
    loop {
        let message = app::view(&model);
        match app::update(&message, model) {
            None => return,
            Some(next_model) => model = next_model,
        }
    }
}

struct Programs {
    floor_program: FloorProgram,
    patch_program: PatchProgram,
}

impl Programs {
    fn draw<T: Surface>(&self, surface: &mut T, view: &[[f32; 4]; 4], projection: &[[f32; 4]; 4]) {
        self.patch_program.draw(surface, view, projection);
        self.floor_program.draw(surface, view, projection);
    }
}

fn run_in_vr(shape_list: ShapeList) {
    let vr_option = System::up().ok();
    if vr_option.is_some() {
        let vr: System = vr_option.unwrap();
        let sleep_time = time::Duration::from_millis(15);

        let render_size = vr.get_render_size();
        println!("{:?}", render_size);

        let can_render = vr.get_can_render();
        println!("Can render {}", can_render);

        let display: Display = glium::glutin::WindowBuilder::new()
            .with_title("vrcounter")
            .with_depth_buffer(24)
            .build_glium()
            .unwrap();

        let left_buffers = EyeBuffers::new(&display, &render_size);
        let mut left_frame = SimpleFrameBuffer::with_depth_buffer(
            &display,
            left_buffers.color.to_color_attachment(),
            left_buffers.depth.to_depth_attachment())
            .unwrap();
        let left_projection = vr.get_left_projection();

        let right_buffers = EyeBuffers::new(&display, &render_size);
        let mut right_frame = SimpleFrameBuffer::with_depth_buffer(
            &display,
            right_buffers.color.to_color_attachment(),
            right_buffers.depth.to_depth_attachment())
            .unwrap();
        let right_projection = vr.get_right_projection();

        let programs = Programs {
            patch_program: PatchProgram::new(&display, shape_list),
            floor_program: FloorProgram::new(&display)
        };
        let clear_color = (0.05, 0.05, 0.08, 1.0);
        let clear_depth = 1.0;

        'render: loop {
            let poses = vr.await_poses();
            let world_to_hmd = poses.get_world_to_hmd_matrix();
            //println!("World to hmd: {:?}", world_to_hmd);

            let mut target = display.draw();
            target.clear_color_and_depth(clear_color, clear_depth);
            programs.draw(&mut target, &world_to_hmd, &left_projection);
            target.finish().unwrap();

            left_frame.clear_color_and_depth(clear_color, clear_depth);
            programs.draw(&mut left_frame, &world_to_hmd, &left_projection);
            vr.submit_left_texture(left_buffers.color.get_id() as usize);

            right_frame.clear_color_and_depth(clear_color, clear_depth);
            programs.draw(&mut right_frame, &world_to_hmd, &right_projection);
            vr.submit_right_texture(right_buffers.color.get_id() as usize);

            for ev in display.poll_events() {
                match ev {
                    glium::glutin::Event::Closed => break 'render,
                    Event::KeyboardInput(ElementState::Pressed, 1, _) => break 'render,
                    _ => ()
                }
            }
            thread::sleep(sleep_time);
        }
    }
}

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

    pub fn submit_left_texture(&self, texture_id: usize) {
        self.submit_texture(Eye::Left, texture_id);
    }
    pub fn submit_right_texture(&self, texture_id: usize) {
        self.submit_texture(Eye::Right, texture_id)
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

fn raw4_from_nmatrix4(m: &nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [m.m11, m.m21, m.m31, m.m41],
        [m.m12, m.m22, m.m32, m.m42],
        [m.m13, m.m23, m.m33, m.m43],
        [m.m14, m.m24, m.m34, m.m44],
    ]
}

fn nmatrix4_from_steam34(r: &[[f32; 4]; 3]) -> nalgebra::Matrix4<f32> {
    nalgebra::Matrix4::new(
        r[0][0], r[1][0], r[2][0], 0.0,
        r[0][1], r[1][1], r[2][1], 0.0,
        r[0][2], r[1][2], r[2][2], 0.0,
        r[0][3], r[1][3], r[2][3], 1.0).transpose()
}

fn nmatrix4_from_steam44(r: &[[f32; 4]; 4]) -> nalgebra::Matrix4<f32> {
    nalgebra::Matrix4::new(
        r[0][0], r[1][0], r[2][0], r[3][0],
        r[0][1], r[1][1], r[2][1], r[3][1],
        r[0][2], r[1][2], r[2][2], r[3][2],
        r[0][3], r[1][3], r[2][3], r[3][3],
    ).transpose()
}
