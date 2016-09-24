extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use super::*;
use std::rc::Rc;
use std::fmt::Debug;

pub struct InFrontOfWailerModel<A, B> where A: Clone + Debug + 'static, B: Clone + Debug + 'static,
{
    frame: Frame,
    offset: Offset,
    a_wailing: Wailing<A>,
    b_wailing: Wailing<B>,
    suboffsets: (Offset, Offset)
}

#[derive(Clone)]
pub struct InFrontOfWailer<A, B, O> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, O: Clone + Debug + 'static
{
    a_wail: Rc<Subwailer<A>>,
    b_wail: Rc<Subwailer<B>>,
    adapt: Rc<Fn(Biopt<A, B>) -> O>,
}

impl<A, B, O> InFrontOfWailer<A, B, O> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, O: Clone + Debug + 'static
{
    pub fn new<F>(left: Rc<Subwailer<A>>, right: Rc<Subwailer<B>>, adapt: F) -> Self
        where F: 'static + Fn(Biopt<A, B>) -> O
    {
        InFrontOfWailer { a_wail: left, b_wail: right, adapt: Rc::new(adapt) }
    }
}

fn add_offsets(a: &Offset, b: &Offset) -> Offset {
    a.shift(b.x, b.y, b.z)
}

impl<A, B, O> Wailer<O> for InFrontOfWailer<A, B, O> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, O: Clone + Debug + 'static
{
    type Mdl = InFrontOfWailerModel<A, B>;

    fn report(&self, model: &InFrontOfWailerModel<A, B>) -> Vec<O> {
        let mut out = Vec::new();
        for a in model.a_wailing.report() {
            out.push((*self.adapt)(Biopt::SomeA(a)));
        }
        for b in model.b_wailing.report() {
            out.push((*self.adapt)(Biopt::SomeB(b)));
        }
        out
    }
    fn update(&self, model: &mut InFrontOfWailerModel<A, B>, message: &WailerIn) {
        match message {
            &WailerIn::Offset(offset) => {
                let (a_suboffset, b_suboffset) = model.suboffsets;
                let (a_offset, b_offset) = (add_offsets(&a_suboffset, &offset),
                                            add_offsets(&b_suboffset, &offset));
                model.offset = offset;
                model.a_wailing.update(&WailerIn::Offset(a_offset));
                model.b_wailing.update(&WailerIn::Offset(b_offset));
            },
            &WailerIn::Hand(hand) => {
                let (a_suboffset, b_suboffset) = model.suboffsets;
                let (a_offset, b_offset) = (add_offsets(&a_suboffset, &model.offset), add_offsets(&b_suboffset, &model.offset));
                model.a_wailing.update(&WailerIn::Hand(hand.minus_offset(&a_offset)));
                model.b_wailing.update(&WailerIn::Hand(hand.minus_offset(&b_offset)));
            }
        }
    }
    fn view(&self, model: &InFrontOfWailerModel<A, B>) -> Vision<WailerIn> {
        let a_vision = model.a_wailing.view();
        let b_vision = model.b_wailing.view();
        let mut vision = Vision::new() as Vision<WailerIn>;
        // TODO Fix vision adapter.
        vision.add_vision(a_vision, |_| None);
        vision.add_vision(b_vision, |_| None);
        vision
    }
    fn init(&self) -> InFrontOfWailerModel<A, B> {
        let mut a_wailing = self.a_wail.as_ref().summon();
        let mut b_wailing = self.b_wail.as_ref().summon();
        let (a_frame, b_frame) = (a_wailing.report_frame(),
                                  b_wailing.report_frame());
        let frame = Frame::from((a_frame.w.max(b_frame.w),
                                 a_frame.h.max(b_frame.h),
                                 a_frame.d.max(b_frame.d)));
        let offset = Offset::default();
        let (a_offset, b_offset) = (Offset::from((0.0, 0.0, 0.05)),
                                    Offset::from((0.0, 0.0, 0.00)));
        a_wailing.update(&WailerIn::Offset(a_offset));
        b_wailing.update(&WailerIn::Offset(b_offset));
        InFrontOfWailerModel {
            frame: frame,
            offset: offset,
            a_wailing: a_wailing,
            b_wailing: b_wailing,
            suboffsets: (a_offset, b_offset)
        }
    }
    fn to_subwail(&self) -> Rc<Subwailer<O>> {
        Rc::new(InFrontOfSubwailer { wailer: self.clone(), wailer_model: None }) as Rc<Subwailer<O>>
    }
}

pub struct InFrontOfSubwailer<A, B, O> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, O: Clone + Debug + 'static {
    wailer: InFrontOfWailer<A, B, O>,
    wailer_model: Option<InFrontOfWailerModel<A, B>>,
}

impl<A, B, O> Subwailer<O> for InFrontOfSubwailer<A, B, O> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, O: Clone + Debug + 'static {
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
                subwail: Box::new(InFrontOfSubwailer {
                    wailer: self.wailer.clone(),
                    wailer_model: Some(self.wailer.init())
                }) as Box<Subwailer<O>>
            }
        }
    }
}

