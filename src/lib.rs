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
mod vr_user;
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
mod demon;
mod demonoid;
mod star;
mod report;

pub fn main() {
    let viewer = viewer::Viewer::start();
    let app = app::start(viewer.clone());
    if os::is_windows() {
        vr_user::run(viewer.clone(), app.clone());
    } else {
        gl_user::run(viewer.clone(), app.clone());
    }
    app::stop(app);
    viewer.stop();
}
