#[macro_use] extern crate glium;

mod world;
mod mat;
mod cam;

use glium::glutin::{Event, ElementState};

fn main() {
    use glium::{DisplayBuild, Surface};
    let display: glium::Display = glium::glutin::WindowBuilder::new()
        .with_title("vrcounter")
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    let room = world::Room::for_display(&display);
    let mut camera = cam::Camera::start();
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        room.draw(&mut target, &camera);
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, 53, _) => return,
                Event::KeyboardInput(ElementState::Pressed, 1, _) => {
                    camera = camera.down();
                },
                Event::KeyboardInput(ElementState::Pressed, 13, _) => {
                    camera = camera.up();
                },
                Event::KeyboardInput(ElementState::Pressed, 0, _) => {
                    camera = camera.left();
                },
                Event::KeyboardInput(ElementState::Pressed, 2, _) => {
                    camera = camera.right();
                },
                Event::KeyboardInput(ElementState::Pressed, 47, _) => {
                    camera = cam::Camera::start();
                },
                Event::KeyboardInput(ElementState::Pressed, 12, _) => {
                    camera = camera.far();
                },
                Event::KeyboardInput(ElementState::Pressed, 14, _) => {
                    camera = camera.near();
                },
                Event::KeyboardInput(ElementState::Pressed, code, _) => println!("{}", code),
                _ => ()
            }
        }
    }
}
