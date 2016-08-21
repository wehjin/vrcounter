#[macro_use] extern crate glium;

mod world;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display: glium::Display = glium::glutin::WindowBuilder::new()
        .with_title("vrcounter")
        .build_glium()
        .unwrap();

    let room = world::Room::for_display(&display);

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        room.draw(&mut target);
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}