extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use std::fmt::Debug;
use super::expand_right::*;
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum WailerIn {
    Offset(Offset),
}

#[derive(Copy, Clone, Debug)]
pub enum WailerOut {
    Frame(Frame),
}

pub enum Biopt<A, B> {
    SomeA(A),
    SomeB(B),
}

pub trait Wailer<E>: Clone where E: Clone + Debug + 'static {
    type Mdl: 'static;

    fn expand_right<B, ENew, BWailer, F>(&self, right_wail: BWailer, adapt: F)
        -> ExpandRightWailer<E, B, ENew>
        where B: Clone + Debug + 'static,
              ENew: Clone + Debug + 'static,
              BWailer: Wailer<B>,
              F: 'static + Fn(Biopt<E, B>) -> ENew {
        ExpandRightWailer::new(self.to_subwail(), right_wail.to_subwail(), adapt)
    }
    fn report(&self, model: &Self::Mdl) -> Vec<E>;
    fn update(&self, model: &mut Self::Mdl, message: &WailerIn);
    fn view(&self, model: &Self::Mdl) -> Vision<WailerIn>;
    fn init(&self) -> Self::Mdl;

    fn to_subwail(&self) -> Rc<Subwailer<E>>;
    fn summon(&self) -> Wailing<E> {
        self.to_subwail().as_ref().summon()
    }
}

// Do not add Clone. We need to box this trait.
pub trait Subwailer<E> {
    fn report(&self) -> Vec<E>;
    fn report_frame(&self) -> Frame;
    fn update(&mut self, message: &WailerIn);
    fn view(&self) -> Vision<WailerIn>;
    fn summon(&self) -> Wailing<E>;
}

pub struct Wailing<E> {
    pub subwail: Box<Subwailer<E>>
}

impl<E> Wailing<E> {
    fn report(&self) -> Vec<E> {
        self.subwail.as_ref().report()
    }
    pub fn report_frame(&self) -> Frame {
        self.subwail.as_ref().report_frame()
    }
    pub fn update(&mut self, message: &WailerIn) {
        self.subwail.as_mut().update(message);
    }
    pub fn view(&self) -> Vision<WailerIn> {
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

impl <E> Subwailer<E> for $subwail {
    fn report(&self) -> Vec<E> {
        if let Some(ref wail_model) = self.wail_model {
            self.wail.report(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn report_frame(&self) -> Frame {
        if let Some(ref wail_model) = self.wail_model {
            wail_model.frame.clone()
        } else {
            panic!("Must summon");
        }
    }
    fn update(&mut self, message: &WailerIn) {
        if let Some(ref mut wail_model) = self.wail_model {
            self.wail.update(wail_model, message);
        } else {
            panic!("Must summon");
        }
    }
    fn view(&self) -> Vision<WailerIn> {
        if let Some(ref wail_model) = self.wail_model {
            self.wail.view(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn summon(&self) -> Wailing<E> {
        if self.wail_model.is_some() {
            panic!("Already summoned");
        } else {
            Wailing {
                subwail: Box::new($subwail {
                    wail: self.wail.clone(),
                    wail_model: Some(self.wail.init())
                }) as Box<Subwailer<E>>
            }
        }
    }
}
)}
