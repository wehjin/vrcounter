extern crate vrcounterlib as lib;

fn main() {
    let vr_option = lib::System::up().ok();
    if vr_option.is_some() {
        let vr = vr_option.unwrap();

        let render_size = vr.get_render_size();
        println!("{:?}", render_size);

        let can_render = vr.get_can_render();
        println!("Can render {}", can_render);

        let poses = vr.await_poses();
        println!("{:?}", poses);
    }

    let mut model = lib::app::Model::init();
    loop {
        let message = lib::app::view(&model);
        match lib::app::update(&message, model) {
            None => return,
            Some(next_model) => model = next_model,
        }
    }
}
