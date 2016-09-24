extern crate cage;
extern crate rand;

use super::*;
use vision::Vision;
use hand::Hand;
use mist::Mist;
use cage::{Frame, Offset, Cage};
use std::rc::Rc;
use std::fmt::Debug;
use common::Wish;

pub enum EnableHandOut {
    Hand(Hand)
}

pub struct EnableHandModel<A> where A: Clone + Debug + 'static
{
    frame: Frame,
    a_wailing: Wailing<A>,
    offset: Offset,
    mist_id: u64
}

#[derive(Clone)]
pub struct EnableHandWailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static
{
    a_wailer: Rc<Subwailer<A>>,
    adapt: Rc<Fn(Biopt<A, EnableHandOut>) -> O>,
}

impl<A, O> EnableHandWailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static
{
    pub fn new<F>(a_wailer: Rc<Subwailer<A>>, adapt: F) -> Self where F: 'static + Fn(Biopt<A, EnableHandOut>) -> O {
        EnableHandWailer { a_wailer: a_wailer, adapt: Rc::new(adapt) }
    }
}

impl<A, O> Wailer<O> for EnableHandWailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static {
    type Mdl = EnableHandModel<A>;

    fn report(&self, model: &EnableHandModel<A>) -> Vec<O> {
        let mut out = Vec::new();
        for a in model.a_wailing.report() {
            out.push((*self.adapt)(Biopt::SomeA(a)));
        }
        // TODO Add hand reports?
        out
    }
    fn update(&self, model: &mut EnableHandModel<A>, message: &WailerIn) {
        match message {
            &WailerIn::Offset(offset) => {
                model.offset = offset;
                model.a_wailing.update(&WailerIn::Offset(offset));
            },
            &WailerIn::Hand(hand) => {
                println!("Updated hand: {:?}", hand);
                // TODO create hand report.
            }
        }
    }
    fn view(&self, model: &EnableHandModel<A>) -> Vision<WailerIn> {
        let a_vision = model.a_wailing.view();
        let mut vision = Vision::new() as Vision<WailerIn>;
        let frame = model.a_wailing.report_frame();
        let offset = model.offset;
        let cage = Cage::from((frame, offset));
        let mist = Mist::new(model.mist_id, cage);
        vision.add_mist(mist, |wish| match wish {
            Wish::SenseHand(hand) => {
                println!("Adapted hand: {:?}", hand);
                Some(WailerIn::Hand(hand))
            },
            _ => None
        });
        vision.add_vision(a_vision, |_| None);
        vision
    }
    fn init(&self) -> EnableHandModel<A> {
        let mut a_wailing = self.a_wailer.as_ref().summon();
        let frame = a_wailing.report_frame();
        let offset = cage::OFFSET_ZERO;
        a_wailing.update(&WailerIn::Offset(offset));
        EnableHandModel { frame: frame, offset: offset, a_wailing: a_wailing, mist_id: rand::random::<u64>() }
    }
    fn to_subwail(&self) -> Rc<Subwailer<O>> {
        Rc::new(EnableHandSubwailer { wailer: self.clone(), wailer_model: None }) as Rc<Subwailer<O>>
    }
}

pub struct EnableHandSubwailer<A, O> where A: Clone + Debug + 'static, O: Clone + Debug + 'static {
    wailer: EnableHandWailer<A, O>,
    wailer_model: Option<EnableHandModel<A>>,
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
