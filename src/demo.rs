extern crate vrcounter;
extern crate cage;

use vrcounter::IdSource;
use vrcounter::Summoner;
use vrcounter::Wish;
use vrcounter::Vision;
use vrcounter::Star;
use std::sync::Arc;

#[derive(Clone)]
pub struct Model;

#[derive(Clone)]
pub struct Message;

#[derive(Clone)]
pub struct Outcome;

#[derive(Clone)]
struct MyStar;

impl Star for MyStar {
    type Mdl = Model;
    type Msg = Message;
    type Out = Outcome;

    fn init(&self) -> (Self::Mdl, Vec<Wish>) {
        use std::rc::Rc;
        (Model, vec![Wish::SummonStar(Rc::new(summon))])
    }

    fn update(&self, _: Self::Msg, _: &Self::Mdl) -> (Option<Self::Mdl>, Vec<Wish>, Vec<Self::Out>) {
        (None, vec![], vec![])
    }

    fn view(&self, _: &Self::Mdl) -> Vision<Self::Msg> {
        Vision::create(|_| Message)
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
}

fn main() {
    let star_builder = Arc::new(|| MyStar);
    vrcounter::start(star_builder)
}
