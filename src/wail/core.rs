extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use std::fmt::Debug;
use super::expand_right::*;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum WailIn {
    Offset(Offset),
}

#[derive(Copy, Clone, Debug)]
pub enum WailOut {
    Frame(Frame),
}

pub trait Wail: Clone + Debug {
    type Mdl: 'static;

    fn expand_right<TRight: Wail>(&self, right_wail: TRight) -> ExpandRightWail {
        ExpandRightWail::new(self.to_subwail(), right_wail.to_subwail())
    }
    fn update(&self, model: &mut Self::Mdl, message: &WailIn);
    fn view(&self, model: &Self::Mdl) -> Vision<WailIn>;
    fn init(&self) -> Self::Mdl;

    fn to_subwail(&self) -> Rc<Subwail>;
    fn summon(&self) -> Wailing {
        self.to_subwail().as_ref().summon()
    }
}

// Do not add Clone. We need to box this trait.
pub trait Subwail: Debug {
    fn report_frame(&self) -> Frame;
    fn update(&mut self, message: &WailIn);
    fn view(&self) -> Vision<WailIn>;
    fn summon(&self) -> Wailing;
}

#[derive(Debug)]
pub struct Wailing {
    pub subwail: Box<Subwail>
}

impl Wailing {
    pub fn report_frame(&self) -> Frame {
        self.subwail.as_ref().report_frame()
    }
    pub fn update(&mut self, message: &WailIn) {
        self.subwail.as_mut().update(message);
    }
    pub fn view(&self) -> Vision<WailIn> {
        self.subwail.as_ref().view()
    }
}

macro_rules! subwail {
($subwail:ident, $wail:ident, $model:ident) => (
#[derive(Debug)]
pub struct $subwail {
    wail: $wail,
    wail_model: Option<$model>,
}

impl Subwail for $subwail {
    fn report_frame(&self) -> Frame {
        if let Some(ref wail_model) = self.wail_model {
            wail_model.frame.clone()
        } else {
            panic!("Must summon");
        }
    }
    fn update(&mut self, message: &WailIn) {
        if let Some(ref mut wail_model) = self.wail_model {
            self.wail.update(wail_model, message);
        } else {
            panic!("Must summon");
        }
    }
    fn view(&self) -> Vision<WailIn> {
        if let Some(ref wail_model) = self.wail_model {
            self.wail.view(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn summon(&self) -> Wailing {
        if self.wail_model.is_some() {
            panic!("Already summoned");
        } else {
            Wailing {
                subwail: Box::new($subwail {
                    wail: self.wail.clone(),
                    wail_model: Some(self.wail.init())
                }) as Box<Subwail>
            }
        }
    }
}
)}
