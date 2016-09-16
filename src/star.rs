use vision::Vision;
use report::Well;
use std::rc::Rc;

pub trait Star: Clone {
    type Mdl: Clone;
    type Msg: Clone;
    type Out: Clone;

    fn init(&self) -> Self::Mdl;
    fn view(&self, &Self::Mdl) -> Vision<Self::Msg>;
    fn update(&self, &Self::Mdl, Self::Msg) -> Option<Self::Mdl>;
    fn report<T>(&self, &Self::Mdl, &mut Well<Self::Out, T>) {}
}

#[derive(Clone, Debug)]
pub struct Substar<S: Star> {
    star_rc: Rc<S>,
    model: S::Mdl,
}

impl<S: Star> Substar<S> {
    pub fn new(star_rc: Rc<S>) -> Self {
        Substar {
            star_rc: star_rc.clone(),
            model: star_rc.as_ref().init(),
        }
    }
    pub fn view(&self) -> Vision<S::Msg> {
        self.star_rc.as_ref().view(&self.model)
    }
    pub fn update(&self, message: S::Msg) -> Option<Self> {
        if let Some(new_model) = self.star_rc.as_ref().update(&self.model, message) {
            Some(Substar { star_rc: self.star_rc.clone(), model: new_model })
        } else {
            None
        }
    }
}
