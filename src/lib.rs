#[macro_use]
extern crate glium;
extern crate openvr;
extern crate openvr_sys;
extern crate nalgebra;
extern crate image;
extern crate rusttype;
extern crate unicode_normalization;
extern crate cage;

pub mod app;
pub mod atlas;
pub mod color;
pub mod gl_user;
pub mod glyffin;
pub mod sigil;
pub mod star;

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
mod vr_user;
mod eyebuffers;
mod common;
mod os;
mod shape;
mod viewer;
mod mist;
mod patch;
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

use std::sync::Arc;

pub use common::IdSource;
pub use common::Wish;
pub use summoner::Summoner;
pub use vision::Vision;
pub use report::Well;
pub use star::*;
pub use patch::Patch;
pub use patch::PatchPosition;
pub use mist::Mist;
pub use beat::Beat;
pub use hand::Hand;
pub use viewer::Viewer;
pub use sigil::Sigil;

// TODO delete this
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
