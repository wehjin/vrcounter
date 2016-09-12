use std::rc::Rc;
use vision::Vision;
use report::Report;
use common::Wish;


pub trait Star: Clone {
    type Mdl: Clone;
    type Msg: Clone;
    type Out: Clone;

    fn init(&self) -> (Self::Mdl, Vec<Wish>);
    fn update(&self, Self::Msg, &Self::Mdl) -> Report<Self::Mdl, Self::Out>;
    fn view(&self, &Self::Mdl) -> Vision<Self::Msg>;
}

#[derive(Clone)]
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

impl<Mdl, Msg, Out> Star for SeedStar<Mdl, Msg, Out> where Mdl: Clone, Msg: Clone, Out: Clone
{
    type Mdl = Mdl;
    type Msg = Msg;
    type Out = Out;

    fn init(&self) -> (Mdl, Vec<Wish>) {
        (*self.init)()
    }

    fn update(&self, message: Msg, model: &Mdl) -> Report<Mdl, Out> {
        (*self.update)(message, model)
    }

    fn view(&self, model: &Mdl) -> Vision<Msg> {
        (*self.view)(model)
    }
}
