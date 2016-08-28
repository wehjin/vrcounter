extern crate vrcounterlib as lib;
extern crate glium;
use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState};
use std::{thread,time};

fn main() {
    let vr_option = lib::System::up().ok();
    if vr_option.is_some() {
        let vr : lib::System = vr_option.unwrap();
        let sleep_time = time::Duration::from_millis(15);

        let render_size = vr.get_render_size();
        println!("{:?}", render_size);

        let can_render = vr.get_can_render();
        println!("Can render {}", can_render);

        let left_projection = vr.get_left_projection();
        println!("Left projection: {:?}", left_projection);

        let display: glium::Display = glium::glutin::WindowBuilder::new()
            .with_title("vrcounter")
            .with_depth_buffer(24)
            .build_glium()
            .unwrap();

        let room: lib::world::Room = lib::world::Room::for_display(&display);

        'render: loop {
            let poses = vr.await_poses();
            poses.audit();

            let world_to_hmd = poses.get_world_to_hmd_matrix();
            println!("World to hmd: {:?}", world_to_hmd);

            let mut target = display.draw();
            target.clear_color_and_depth((0.15, 0.15, 0.18, 1.0), 1.0);
            room.draw2(&mut target, &world_to_hmd, &left_projection);
            target.finish().unwrap();

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
