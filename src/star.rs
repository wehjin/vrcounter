use std::rc::Rc;
use vision::Vision;
use report::Report;
use common::Wish;

pub struct SeedStar<Mdl, Msg, Out> where Mdl: Clone {
    pub init: Rc<Fn() -> (Mdl, Vec<Wish>)>,
    pub update: Rc<Fn(Msg, &Mdl) -> Report<Mdl, Out>>,
    pub view: Rc<Fn(&Mdl) -> Vision<Msg>>,
}

impl<Mdl, Msg, Out> SeedStar<Mdl, Msg, Out> where Mdl: Clone {
    pub fn create<In, Up, Vw>(init: In, update: Up, view: Vw) -> Self where In: Fn() -> (Mdl, Vec<Wish>) + 'static,
                                                                            Up: Fn(Msg, &Mdl) -> Report<Mdl, Out> + 'static,
                                                                            Vw: Fn(&Mdl) -> Vision<Msg> + 'static {
        SeedStar { init: Rc::new(init), update: Rc::new(update), view: Rc::new(view) }
    }
}
