use vision::Vision;
use report::Well;
use std::rc::Rc;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct CompositeSubstar<S: Star>
    where S: Debug, < S as Star >::Mdl: Debug, < S as Star >::Msg: 'static {
    substars: Vec<Substar<S>>
}

impl<S: Star> CompositeSubstar<S>
where S: Debug, < S as Star >::Mdl: Debug, < S as Star >::Msg: 'static {
    pub fn init(substar_inits: Vec<Rc<S>>) -> Self {
        let mut substars = Vec::new();
        for star in substar_inits {
            substars.push(Substar::init(star));
        }
        CompositeSubstar {
            substars: substars
        }
    }
    pub fn view(&self) -> Vision<()> {
        let mut vision = Vision::new();
        for substar in &self.substars {
            vision.add_vision(substar.view(), |_| None);
        };
        vision
    }
    pub fn update(&self, _: S::Msg) -> Self {
        self.clone()
    }
}


#[derive(Clone, Debug)]
pub struct Substar<S: Star> {
    star_rc: Rc<S>,
    star_model: S::Mdl,
}

impl<S: Star> Substar<S> {
    pub fn init(star_rc: Rc<S>) -> Self {
        Substar {
            star_rc: star_rc.clone(),
            star_model: star_rc.as_ref().init(),
        }
    }
    pub fn view(&self) -> Vision<S::Msg> {
        self.star_rc.as_ref().view(&self.star_model)
    }
    pub fn update(&self, message: &S::Msg) -> Self {
        let new_star_model = self.star_rc.as_ref().update(&self.star_model, message);
        Substar { star_rc: self.star_rc.clone(), star_model: new_star_model }
    }
}

pub trait Star: Clone {
    type Mdl: Clone;
    type Msg: Clone;
    type Out: Clone;

    fn init(&self) -> Self::Mdl;
    fn view(&self, &Self::Mdl) -> Vision<Self::Msg>;
    fn update(&self, &Self::Mdl, &Self::Msg) -> Self::Mdl;
    fn report<T>(&self, &Self::Mdl, &mut Well<Self::Out, T>) {}
}
