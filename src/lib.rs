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
mod vr;

use glium::{DisplayBuild};
use glium::framebuffer::{SimpleFrameBuffer, ToColorAttachment, ToDepthAttachment};
use glium::glutin::{Event, ElementState, WindowBuilder};
use programs::Programs;
use viewer::ActiveViewer;
use std::rc::Rc;
use app::{Message as AppMessage};
use std::sync::mpsc::{Sender};
use std::time::{Instant, Duration};
use hmd::Hmd;
use std::borrow::Borrow;
use vr::System;
use openvr::render_models::IVRRenderModels;

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
    println!("Can render {}", vr.get_can_render());


    let render_models: IVRRenderModels = openvr::subsystems::render_models().unwrap();
    let count = render_models.get_count();
    println!("Render model names: {:?}", count);
    for index in 0..count {
        let name = render_models.get_name(index);
        println!("{} {}", index+1, name);
    }

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
