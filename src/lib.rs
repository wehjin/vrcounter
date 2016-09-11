#[macro_use] extern crate glium;
extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;
extern crate image;
extern crate rusttype;
extern crate unicode_normalization;
extern crate cage;

mod hand;
mod hand_program;
mod keymap;
mod patch_program;
mod floor_program;
mod mist_program;
mod controller_program;
mod programs;
mod mat;
mod cam;
mod gl_user;
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
mod vr;

use glium::{DisplayBuild};
use glium::framebuffer::{SimpleFrameBuffer, ToColorAttachment, ToDepthAttachment};
use glium::glutin::{Event, ElementState, WindowBuilder};
use programs::Programs;
use programs::SCREEN_TO_WORLD;
use viewer::Viewer;
use std::rc::Rc;
use app::{Message as AppMessage};
use std::sync::mpsc::{Sender};
use std::time::{Instant, Duration};
use hmd::Hmd;
use std::borrow::Borrow;
use vr::System;
use hand::Hand;
use cage::Offset;

pub fn main() {
    let size = std::mem::size_of::<openvr_sys::TrackedDevicePose_t>();
    println!("Size of TrackedDevicePose: {}", size);

    let viewer = Viewer::start();
    let app = app::start(viewer.clone());
    if os::is_windows() {
        run_in_vr(viewer.clone(), app.clone());
    } else {
        gl_user::run(viewer.clone(), app.clone());
    }
    app::stop(app);
    viewer.stop();
}

fn run_in_vr(viewer: Viewer, app: Sender<AppMessage>) {
    use programs::HandType;

    let vr_option = System::up().ok();
    if vr_option.is_none() {
        return;
    }

    let vr: System = vr_option.unwrap();
    println!("Can render {}", vr.get_can_render());


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
    let mut programs = Programs::new(display.clone(), viewer.clone(), HandType::Vive);

    let mut frame_instant = Instant::now();
    let frame_duration = Duration::from_millis(300);

    let poses = vr.await_poses();
    poses.audit();

    'render: loop {
        let poses = vr.await_poses();
        let world_to_hmd = poses.get_world_to_hmd_matrix();

        let controller_matrix_option = poses.get_controller_to_world_matrix();
        programs.set_controller_model_matrix(&controller_matrix_option);
        if let Some(matrix) = controller_matrix_option {
            let position = (matrix[3][0] - SCREEN_TO_WORLD[3][0],
                            matrix[3][1] - SCREEN_TO_WORLD[3][1],
                            matrix[3][2] - SCREEN_TO_WORLD[3][2]);
            viewer.set_hand(Hand { offset: Offset::from(position) });
        }

        hmd.draw(&programs, &world_to_hmd, display.borrow(), &mut left_frame, &mut right_frame);

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => break 'render,
                Event::KeyboardInput(ElementState::Pressed, 1, _) => break 'render,
                _ => ()
            }
        }
        match vr.poll_next_event() {
            Some(vr_event) => {
                println!("{:?}", vr_event);
            }
            None => ()
        }

        if Instant::now().duration_since(frame_instant) > frame_duration {
            frame_instant = Instant::now();
            app.send(AppMessage::EmitAnimationFrame).unwrap_or(());
        }
    }
}
