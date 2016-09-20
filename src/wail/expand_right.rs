extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use super::*;
use std::rc::Rc;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ExpandRightWailerModel<A, B> where A: Clone + Debug + 'static, B: Clone + Debug + 'static,
{
    frame: Frame,
    left_wailing: Wailing<A>,
    right_wailing: Wailing<B>,
    base_offsets: (Offset, Offset)
}

#[derive(Clone, Debug)]
pub struct ExpandRightWailer<A, B, E> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, E: Clone + Debug + 'static {
    left_wail: Rc<Subwailer<A>>,
    right_wail: Rc<Subwailer<B>>,
    report: Vec<E>
}

impl<A, B, E> ExpandRightWailer<A, B, E> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, E: Clone + Debug + 'static {
    pub fn new(left: Rc<Subwailer<A>>, right: Rc<Subwailer<B>>) -> Self {
        ExpandRightWailer { left_wail: left, right_wail: right, report: vec![] }
    }
}

fn add_offsets(a: &Offset, b: &Offset) -> Offset {
    a.shift(b.x, b.y, b.z)
}

impl<A, B, E> Wailer<E> for ExpandRightWailer<A, B, E> where A: Clone + Debug + 'static, B: Clone + Debug + 'static, E: Clone + Debug + 'static {
    type Mdl = ExpandRightWailerModel<A, B>;

    fn report(&self, model: &ExpandRightWailerModel<A, B>) -> Vec<E> {
        vec![]
    }
    fn update(&self, model: &mut ExpandRightWailerModel<A, B>, message: &WailerIn) {
        match message {
            &WailerIn::Offset(offset) => {
                let (left_base, right_base) = model.base_offsets;
                let (left_full, right_full) = (add_offsets(&left_base, &offset),
                                               add_offsets(&right_base, &offset));
                model.left_wailing.update(&WailerIn::Offset(left_full));
                model.right_wailing.update(&WailerIn::Offset(right_full));
            }
        }
    }
    fn view(&self, model: &ExpandRightWailerModel<A, B>) -> Vision<WailerIn> {
        let left_vision = model.left_wailing.view();
        let right_vision = model.right_wailing.view();
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
        // TODO Deal with mis-matched y and z in offsets.
        let right_offset = Offset::from((frame.w / 2.0 - right_frame.w / 2.0, 0.0, 0.0));
        let left_offset = Offset::from((-frame.w / 2.0 + left_frame.w / 2.0, 0.0, 0.0));
        left_wailing.update(&WailerIn::Offset(left_offset));
        right_wailing.update(&WailerIn::Offset(right_offset));
        ExpandRightWailerModel {
            frame: frame,
            left_wailing: left_wailing,
            right_wailing: right_wailing,
            base_offsets: (left_offset, right_offset)
        }
    }
    fn to_subwail(&self) -> Rc<Subwailer<E>> {
        Rc::new(ExpandRightSubwailer { wail: self.clone(), wail_model: None }) as Rc<Subwailer<E>>
    }
}

#[derive(Debug)]
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

