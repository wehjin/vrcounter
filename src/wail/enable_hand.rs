extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use super::*;
use std::rc::Rc;
use std::fmt::Debug;

pub struct EnableHandWailerModel<A> where A: Clone + Debug + 'static
{
    frame: Frame,
    a_wailing: Wailing<A>,
    base_offset: Offset
}

#[derive(Clone)]
pub struct EnableHandWailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static
{
    a_wailer: Rc<Subwailer<A>>,
    adapt: Rc<Fn(A) -> O>,
}

impl<A, O> EnableHandWailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static
{
    pub fn new<F>(a_wailer: Rc<Subwailer<A>>, adapt: F) -> Self where F: 'static + Fn(A) -> O {
        EnableHandWailer { a_wailer: a_wailer, adapt: Rc::new(adapt) }
    }
}

impl<A, O> Wailer<O> for EnableHandWailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static {
    type Mdl = EnableHandWailerModel<A>;

    fn report(&self, model: &EnableHandWailerModel<A>) -> Vec<O> {
        let mut out = Vec::new();
        for a in model.a_wailing.report() {
            out.push((*self.adapt)(a));
        }
        // TODO Add hand reports?
        out
    }
    fn update(&self, model: &mut EnableHandWailerModel<A>, message: &WailerIn) {
        match message {
            &WailerIn::Offset(offset) => {
                let a_base_offset = model.base_offset;
                let a_offset = a_base_offset.shift(offset.x, offset.y, offset.z);
                model.a_wailing.update(&WailerIn::Offset(a_offset));
            }
        }
    }
    fn view(&self, model: &EnableHandWailerModel<A>) -> Vision<WailerIn> {
        let a_vision = model.a_wailing.view();
        let mut vision = Vision::new() as Vision<WailerIn>;
        // TODO Add Mist to vision
        vision.add_vision(a_vision, |_| None);
        vision
    }
    fn init(&self) -> EnableHandWailerModel<A> {
        let mut a_wailing = self.a_wailer.as_ref().summon();
        let a_frame = a_wailing.report_frame();
        let frame = Frame::from((a_frame.w, a_frame.h, a_frame.d));
        let a_offset = Offset::from((0.0, 0.0, 0.0));
        a_wailing.update(&WailerIn::Offset(a_offset));
        EnableHandWailerModel { frame: frame, a_wailing: a_wailing, base_offset: a_offset }
    }
    fn to_subwail(&self) -> Rc<Subwailer<O>> {
        Rc::new(EnableHandSubwailer { wailer: self.clone(), wailer_model: None }) as Rc<Subwailer<O>>
    }
}

pub struct EnableHandSubwailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static {
    wailer: EnableHandWailer<A, O>,
    wailer_model: Option<EnableHandWailerModel<A>>,
}

impl<A, O> Subwailer<O> for EnableHandSubwailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static {
    fn report(&self) -> Vec<O> {
        if let Some(ref wail_model) = self.wailer_model {
            self.wailer.report(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn report_frame(&self) -> Frame {
        if let Some(ref wail_model) = self.wailer_model {
            wail_model.frame.clone()
        } else {
            panic!("Must summon");
        }
    }
    fn update(&mut self, message: &WailerIn) {
        if let Some(ref mut wail_model) = self.wailer_model {
            self.wailer.update(wail_model, message);
        } else {
            panic!("Must summon");
        }
    }
    fn view(&self) -> Vision<WailerIn> {
        if let Some(ref wail_model) = self.wailer_model {
            self.wailer.view(wail_model)
        } else {
            panic!("Must summon");
        }
    }
    fn summon(&self) -> Wailing<O> {
        if self.wailer_model.is_some() {
            panic!("Already summoned");
        } else {
            Wailing {
                subwail: Box::new(EnableHandSubwailer {
                    wailer: self.wailer.clone(),
                    wailer_model: Some(self.wailer.init())
                }) as Box<Subwailer<O>>
            }
        }
    }
}
