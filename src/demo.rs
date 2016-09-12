extern crate vrcounter;
extern crate cage;

use vrcounter::IdSource;
use vrcounter::Summoner;
use vrcounter::Wish;
use vrcounter::Report;
use vrcounter::Vision;
use vrcounter::SeedStar;
use std::sync::Arc;
use vrcounter::star::Star;

#[derive(Clone)]
pub struct Model;

#[derive(Clone)]
pub struct Message;

#[derive(Clone)]
pub struct Outcome;

struct MyStar;

//impl Star for MyStar {
//    fn init(&self) -> (Mdl, Vec<Wish>) {
//        unimplemented!()
//    }
//
//    fn update(&self, _: Msg, _: &Mdl) -> Report<Mdl, Out> {
//        unimplemented!()
//    }
//
//    fn view(&self, _: &Mdl) -> Vision<Msg> {
//        unimplemented!()
//    }
//}

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

fn main() {
    let star_builder = Arc::new(|| SeedStar::create(init, update, view));
    vrcounter::start(star_builder)
}
