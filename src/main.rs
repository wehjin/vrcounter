#[macro_use] extern crate glium;

mod world;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display: glium::Display = glium::glutin::WindowBuilder::new()
        .with_title("vrcounter")
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    let room = world::Room::for_display(&display);

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

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
