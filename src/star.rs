use std::rc::Rc;
use vision::Vision;
use report::Report;
use common::Wish;

pub trait Star {
    type Mdl: Clone;
    type Msg;
    type Out;

    fn init() -> Self::Mdl;
    fn update(Self::Msg, &Self::Mdl) -> Report<Self::Mdl, Self::Out>;
    fn view(&Self::Mdl) -> Vision<Self::Msg>;
}


pub struct SeedStar<Mdl, Msg, Out> where Mdl: Clone {
    pub init: Rc<Fn() -> (Mdl, Option<Wish>)>,
    pub update: Rc<Fn(Msg, &Mdl) -> Report<Mdl, Out>>,
    pub view: Rc<Fn(&Mdl) -> Vision<Msg>>,
}

impl<Mdl, Msg, Out> SeedStar<Mdl, Msg, Out> where Mdl: Clone {
    pub fn create<F, G, H>(init: F, update: G, view: H) -> Self where F: Fn() -> (Mdl, Option<Wish>) + 'static,
                                                                      G: Fn(Msg, &Mdl) -> Report<Mdl, Out> + 'static,
                                                                      H: Fn(&Mdl) -> Vision<Msg> + 'static {
        SeedStar { init: Rc::new(init), update: Rc::new(update), view: Rc::new(view) }
    }
}
