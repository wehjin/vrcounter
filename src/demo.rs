extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::IdSource;
use vrcounter::Summoner;
use vrcounter::Wish;
use vrcounter::Vision;
use vrcounter::Star;
use std::sync::Arc;
use cage::Cage;

#[derive(Clone)]
pub struct Model {
    pub color: [f32; 4],
    pub mist_id: u64,
    pub patch_id: u64
}

#[derive(Clone)]
pub enum Message {
    Ignore,
    Toggle
}

#[derive(Clone)]
pub struct Outcome;

#[derive(Clone)]
struct MyStar;

impl Star for MyStar {
    type Mdl = Model;
    type Msg = Message;
    type Out = Outcome;

    fn init(&self) -> (Model, Vec<Wish>) {
        use std::rc::Rc;
        use vrcounter::color::BLUE;

        let patch_id = rand::random::<u64>();
        let mist_id = rand::random::<u64>();
        let model = Model { color: BLUE, mist_id: mist_id, patch_id: patch_id };
        (model, vec![Wish::SummonStar(Rc::new(summon))])
    }

    fn update(&self, message: Self::Msg, model: &Model) -> (Option<Model>, Vec<Wish>, Vec<Outcome>) {
        (None, vec![], vec![])
    }

    fn view(&self, model: &Model) -> Vision<Message> {
        use vrcounter::{Patch, Sigil};
        use vrcounter::Mist;

        let cage = Cage::from((-0.70, -0.50, -0.10, 0.10, 0.00, 0.20));
        let mut vision = Vision::new(|_| Message::Ignore);
        vision.add_patch(Patch::from_cage(&cage, model.color, Sigil::Fill, model.patch_id));
        vision.add_mist(Mist::new(model.mist_id, cage));
        vision
    }
}

fn summon(id_source: &mut IdSource, summoner: &mut Summoner) {
    use cage::Cage;
    use vrcounter::{howl, scream, roar};
    use vrcounter::color::*;
    use vrcounter::Sigil;

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
}

fn main() {
    let star_builder = Arc::new(|| MyStar);
    vrcounter::start(star_builder)
}
