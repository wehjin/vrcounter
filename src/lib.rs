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

use common::IdSource;
use summoner::Summoner;
use common::Wish;
use report::Report;
use vision::Vision;
use star::SeedStar;

#[derive(Clone)]
pub struct Model;

#[derive(Clone)]
pub struct Message;

#[derive(Clone)]
pub struct Outcome;

fn summon(id_source: &mut IdSource, summoner: &mut Summoner) -> u64 {
    use howl;
    use color::*;
    use cage::Cage;
    use patch::Sigil;

    let roar = roar::demo::from(vec![GREEN, RED, BLUE, CYAN, MAGENTA, YELLOW]);
    summoner.summon(id_source, &roar, |_| Outcome);

    let scream_id1 = id_source.id();
    let scream1 = scream::from_color(scream_id1, CYAN);
    let screaming1 = summoner.summon(id_source, &scream1, |_| Outcome);
    let cage1 = Cage::from((-0.3, -0.2, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming1, Wish::FitToCage(Cage::from(cage1)));

    let scream_id2 = id_source.id();
    let scream2 = scream::from_color(scream_id2, MAGENTA);
    let screaming2 = summoner.summon(id_source, &scream2, |_| Outcome);
    let cage2 = Cage::from((-0.4, -0.3, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming2, Wish::FitToCage(Cage::from(cage2)));
    let scream_id3 = id_source.id();
    let screaming3 = summoner.summon(id_source, &scream::from_color(scream_id3, YELLOW), |_| Outcome);
    let cage3 = Cage::from((-0.5, -0.4, -0.25, -0.15, 0.03, 0.03));
    summoner.update_one(screaming3, Wish::FitToCage(Cage::from(cage3)));
    let howls = vec![
        howl::create(id_source.id(), BLUE, Cage::from((-0.70, -0.50, -0.10, 0.10, 0.10, 0.10)), Sigil::Fill),
        howl::create(id_source.id(), RED, Cage::from((-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)), Sigil::Fill),
        howl::create(id_source.id(), GREEN, Cage::from((0.25, 0.75, 0.0, 0.5, -0.01, -0.01)), Sigil::Fill),
        howl::create(id_source.id(), CYAN, Cage::from((-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('J')),
        howl::create(id_source.id(), YELLOW, Cage::from((0.00, 0.06, -0.03, 0.03, 0.005, 0.005)), Sigil::Letter('y')),
    ];
    for howl in &howls {
        summoner.summon(id_source, howl, |_| Outcome);
    }
    let howl_id = id_source.id();
    summoner.summon(id_source, &howl::misty(howl_id, Default::default()), |_| Outcome);

    // TODO remove return from Wish::SummonStar
    0
}

fn init() -> (Model, Vec<Wish>) {
    use std::rc::Rc;
    (Model, vec![Wish::SummonStar(Rc::new(summon))])
}

fn update(_: Message, _: &Model) -> Report<Model, Outcome> {
    Report::Unchanged
}

fn view(_: &Model) -> Vision<Message> {
    Vision::create(|_| Message)
}

pub fn main() {
    use std::sync::Arc;
    let viewer = viewer::Viewer::start();

    let star_builder = Arc::new(|| SeedStar::create(init, update, view));
    let app = app::start(viewer.clone(), star_builder);

    if os::is_windows() {
        vr_user::run(viewer.clone(), app.clone());
    } else {
        gl_user::run(viewer.clone(), app.clone());
    }

    app::stop(app);
    viewer.stop();
}
