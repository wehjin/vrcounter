#[macro_use] extern crate glium;

mod world;
mod mat;
mod cam;
mod app;

fn main() {
    let mut model = app::Model::init();
    loop {
        let message = app::view(&model);
        match app::update(&message, model) {
            None => return,
            Some(next_model) => model = next_model,
        }
    }
}
