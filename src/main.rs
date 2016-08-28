extern crate vrcounterlib as lib;
extern crate glium;
use glium::{DisplayBuild, Surface, Display, GlObject};
use glium::framebuffer::{RenderBuffer,DepthRenderBuffer,SimpleFrameBuffer,ToColorAttachment,ToDepthAttachment};
use glium::glutin::{Event, ElementState};
use std::{thread,time};
use lib::{System, RenderSize};

struct EyeBuffers {
    depth: DepthRenderBuffer,
    color: RenderBuffer,
}
impl EyeBuffers {
    fn new(display:&Display, render_size:&RenderSize) -> Self {
        let depth_buffer: DepthRenderBuffer = glium::framebuffer::DepthRenderBuffer::new(
            display,
            glium::texture::DepthFormat::I24,
            render_size.width, render_size.height).unwrap();
        let color_buffer : RenderBuffer = glium::framebuffer::RenderBuffer::new(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            render_size.width, render_size.height).unwrap();
        EyeBuffers {depth:depth_buffer, color:color_buffer}
    }
}

fn main() {
    let vr_option = lib::System::up().ok();
    if vr_option.is_some() {
        let vr : System = vr_option.unwrap();
        let sleep_time = time::Duration::from_millis(15);

        let render_size : lib::RenderSize = vr.get_render_size();
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

        let room: lib::world::Room = lib::world::Room::for_display(&display);
        let clear_color = (0.05, 0.05, 0.08, 1.0);
        let clear_depth = 1.0;

        'render: loop {
            let poses = vr.await_poses();
            let world_to_hmd = poses.get_world_to_hmd_matrix();
            //println!("World to hmd: {:?}", world_to_hmd);

            let mut target = display.draw();
            target.clear_color_and_depth(clear_color, clear_depth);
            room.draw(&mut target, &world_to_hmd, &left_projection);
            target.finish().unwrap();

            left_frame.clear_color_and_depth(clear_color, clear_depth);
            room.draw(&mut left_frame, &world_to_hmd, &left_projection);
            vr.submit_left_texture(left_buffers.color.get_id() as usize);

            right_frame.clear_color_and_depth(clear_color, clear_depth);
            room.draw(&mut right_frame, &world_to_hmd, &right_projection);
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
    } else {
        let mut model = lib::app::Model::init();
        loop {
            let message = lib::app::view(&model);
            match lib::app::update(&message, model) {
                None => return,
                Some(next_model) => model = next_model,
            }
        }
    }
}
