use std::rc::Rc;
use vision::Vision;
use report::Report;

pub struct Star<Mdl, Msg, Out> where Mdl: Clone {
    pub init: Rc<Fn() -> Mdl>,
    pub update: Rc<Fn(Msg, &Mdl) -> Report<Mdl, Out>>,
    pub view: Rc<Fn(&Mdl) -> Vision<Msg>>,
}

impl<Mdl, Msg, Out> Star<Mdl, Msg, Out> where Mdl: Clone {
    pub fn create<F, G, H>(init: F, update: G, view: H) -> Self where F: Fn() -> Mdl + 'static,
                                                                      G: Fn(Msg, &Mdl) -> Report<Mdl, Out> + 'static,
                                                                      H: Fn(&Mdl) -> Vision<Msg> + 'static {
        Star { init: Rc::new(init), update: Rc::new(update), view: Rc::new(view) }
    }
}
