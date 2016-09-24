extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use super::*;
use std::rc::Rc;
use std::fmt::Debug;

pub struct ExpandRightWailerModel<A, B> where A: Clone + Debug + 'static, B: Clone + Debug + 'static,
{
    frame: Frame,
    offset: Offset,
    a_wailing: Wailing<A>,
    b_wailing: Wailing<B>,
    suboffsets: (Offset, Offset)
}

#[derive(Clone)]
pub struct ExpandRightWailer<A, B, E> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, E: Clone + Debug + 'static {
    left_wail: Rc<Subwailer<A>>,
    right_wail: Rc<Subwailer<B>>,
    adapt: Rc<Fn(Biopt<A, B>) -> E>,
}

impl<A, B, E> ExpandRightWailer<A, B, E> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, E: Clone + Debug + 'static {
    pub fn new<F>(left: Rc<Subwailer<A>>, right: Rc<Subwailer<B>>, adapt: F)
        -> Self where F: 'static + Fn(Biopt<A, B>) -> E
    {
        ExpandRightWailer { left_wail: left, right_wail: right, adapt: Rc::new(adapt) }
    }
}

fn add_offsets(a: &Offset, b: &Offset) -> Offset {
    a.shift(b.x, b.y, b.z)
}

impl<A, B, O> Wailer<O> for ExpandRightWailer<A, B, O> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, O: Clone + Debug + 'static {
    type Mdl = ExpandRightWailerModel<A, B>;

    fn report(&self, model: &ExpandRightWailerModel<A, B>) -> Vec<O> {
        let mut out = Vec::new();
        for a in model.a_wailing.report() {
            out.push((*self.adapt)(Biopt::SomeA(a)));
        }
        for b in model.b_wailing.report() {
            out.push((*self.adapt)(Biopt::SomeB(b)));
        }
        out
    }
    fn update(&self, model: &mut ExpandRightWailerModel<A, B>, message: &WailerIn) {
        match message {
            &WailerIn::Offset(offset) => {
                let (a_suboffset, b_suboffset) = model.suboffsets;
                let (a_offset, b_offset) = (add_offsets(&a_suboffset, &offset), add_offsets(&b_suboffset, &offset));
                model.offset = offset;
                model.a_wailing.update(&WailerIn::Offset(a_offset));
                model.b_wailing.update(&WailerIn::Offset(b_offset));
            },
            &WailerIn::Hand(hand) => {
                // TODO check if hand is in subcages?
                let (a_suboffset, b_suboffset) = model.suboffsets;
                let (a_offset, b_offset) = (add_offsets(&a_suboffset, &model.offset), add_offsets(&b_suboffset, &model.offset));
                model.a_wailing.update(&WailerIn::Hand(hand.minus_offset(&a_offset)));
                model.b_wailing.update(&WailerIn::Hand(hand.minus_offset(&b_offset)));
            }
        }
    }
    fn view(&self, model: &ExpandRightWailerModel<A, B>) -> Vision<WailerIn> {
        let left_vision = model.a_wailing.view();
        let right_vision = model.b_wailing.view();
        let mut vision = Vision::new() as Vision<WailerIn>;
        vision.add_vision(left_vision, |_| None);
        vision.add_vision(right_vision, |_| None);
        vision
    }
    fn init(&self) -> ExpandRightWailerModel<A, B> {
        let mut left_wailing = self.left_wail.as_ref().summon();
        let mut right_wailing = self.right_wail.as_ref().summon();
        let left_frame = left_wailing.report_frame();
        let right_frame = right_wailing.report_frame();
        let frame = Frame::from((left_frame.w + right_frame.w,
                                 left_frame.h.max(right_frame.h),
                                 left_frame.d.max(right_frame.d)));
        let offset = Offset::default();
        // TODO Deal with mis-matched y and z in offsets.
        let right_suboffset = Offset::from((frame.w / 2.0 - right_frame.w / 2.0, 0.0, 0.0));
        let left_suboffset = Offset::from((-frame.w / 2.0 + left_frame.w / 2.0, 0.0, 0.0));
        left_wailing.update(&WailerIn::Offset(left_suboffset));
        right_wailing.update(&WailerIn::Offset(right_suboffset));
        ExpandRightWailerModel {
            frame: frame,
            offset: offset,
            a_wailing: left_wailing,
            b_wailing: right_wailing,
            suboffsets: (left_suboffset, right_suboffset)
        }
    }
    fn to_subwail(&self) -> Rc<Subwailer<O>> {
        Rc::new(ExpandRightSubwailer { wail: self.clone(), wail_model: None }) as Rc<Subwailer<O>>
    }
}

pub struct ExpandRightSubwailer<A, B, E> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, E: Clone + Debug + 'static {
    wail: ExpandRightWailer<A, B, E>,
    wail_model: Option<ExpandRightWailerModel<A, B>>,
}

impl<A, B, E> Subwailer<E> for ExpandRightSubwailer<A, B, E> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, E: Clone + Debug + 'static {
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
                subwail: Box::new(ExpandRightSubwailer {
                    wail: self.wail.clone(),
                    wail_model: Some(self.wail.init())
                }) as Box<Subwailer<E>>
            }
        }
    }
}

