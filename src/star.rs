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
    pub fn init_up(substar_inits: Vec<(Rc<S>, S::Msg)>) -> Self {
        let mut substars = Vec::new();
        for (star, message) in substar_inits {
            substars.push(Substar::init(star).update(message).unwrap());
        }
        CompositeSubstar {
            substars: substars
        }
    }
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
    pub fn update(&self, _: S::Msg) -> Option<Self> {
        None
    }
}


#[derive(Clone, Debug)]
pub struct Substar<S: Star> {
    star_rc: Rc<S>,
    model: S::Mdl,
}

impl<S: Star> Substar<S> {
    pub fn init(star_rc: Rc<S>) -> Self {
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

pub trait Star: Clone {
    type Mdl: Clone;
    type Msg: Clone;
    type Out: Clone;

    fn init(&self) -> Self::Mdl;
    fn view(&self, &Self::Mdl) -> Vision<Self::Msg>;
    fn update(&self, &Self::Mdl, Self::Msg) -> Option<Self::Mdl>;
    fn report<T>(&self, &Self::Mdl, &mut Well<Self::Out, T>) {}
}
