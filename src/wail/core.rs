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

pub trait Wailer: Clone + Debug {
    type Mdl: 'static;

    fn expand_right<TRight: Wailer>(&self, right_wail: TRight) -> ExpandRightWailer {
        ExpandRightWailer::new(self.to_subwail(), right_wail.to_subwail())
    }
    fn update(&self, model: &mut Self::Mdl, message: &WailerIn);
    fn view(&self, model: &Self::Mdl) -> Vision<WailerIn>;
    fn init(&self) -> Self::Mdl;

    fn to_subwail(&self) -> Rc<Subwailer>;
    fn summon(&self) -> Wailing {
        self.to_subwail().as_ref().summon()
    }
}

// Do not add Clone. We need to box this trait.
pub trait Subwailer: Debug {
    fn report_frame(&self) -> Frame;
    fn update(&mut self, message: &WailerIn);
    fn view(&self) -> Vision<WailerIn>;
    fn summon(&self) -> Wailing;
}

#[derive(Debug)]
pub struct Wailing {
    pub subwail: Box<Subwailer>
}

impl Wailing {
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

impl Subwailer for $subwail {
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
    fn summon(&self) -> Wailing {
        if self.wail_model.is_some() {
            panic!("Already summoned");
        } else {
            Wailing {
                subwail: Box::new($subwail {
                    wail: self.wail.clone(),
                    wail_model: Some(self.wail.init())
                }) as Box<Subwailer>
            }
        }
    }
}
)}
