extern crate vrcounterlib as lib;
use std::{thread,time};

fn main() {
    let vr_option = lib::System::up().ok();
    if vr_option.is_some() {
        let vr : lib::System = vr_option.unwrap();

        let render_size = vr.get_render_size();
        println!("{:?}", render_size);

        let can_render = vr.get_can_render();
        println!("Can render {}", can_render);

        let left_projection = vr.get_left_projection();
        println!("Left projection: {:?}", left_projection);

        let one_second = time::Duration::from_secs(1);
        for _ in 0..20 {
            let poses = vr.await_poses();
            poses.audit();

            let world_to_hmd = poses.get_world_to_hmd_matrix();
            println!("World to hmd: {:?}", world_to_hmd);

            thread::sleep(one_second);
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
