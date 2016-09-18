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
pub mod color;
pub mod scream;
pub mod howl;
pub mod roar;
mod mist;
mod patch;
mod app;
mod vision;
mod report;
mod summoner;
mod beat;
mod eye;
mod hmd;
mod constants;
mod poses;
mod vr;
mod demon;
mod demonoid;
pub mod star;

use std::sync::Arc;

pub use common::IdSource;
pub use summoner::Summoner;
pub use common::Wish;
pub use vision::Vision;
pub use report::Well;
pub use star::*;
pub use patch::Sigil;
pub use patch::Patch;
pub use mist::Mist;
pub use beat::Beat;
pub use hand::Hand;
pub use howl::Howl;

pub fn start<S: Star, F>(star_builder: Arc<F>) where S: Clone + 'static,
                                                     F: Fn() -> S + Send + Sync + 'static
{
    let viewer = viewer::Viewer::start();
    let app = app::start(viewer.clone(), star_builder);

    if os::is_windows() {
        vr_user::run(viewer.clone(), app.clone());
    } else {
        gl_user::run(viewer.clone(), app.clone());
    }

    app::stop(app);
    viewer.stop();
}
