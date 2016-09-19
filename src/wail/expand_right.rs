extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use super::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct ExpandRightWailModel
{
    frame: Frame,
    left_wailing: Wailing,
    right_wailing: Wailing,
    base_offsets: (Offset, Offset)
}

#[derive(Clone, Debug)]
pub struct ExpandRightWail {
    left_wail: Rc<Subwail>,
    right_wail: Rc<Subwail>,
}

impl ExpandRightWail {
    pub fn new(left: Rc<Subwail>, right: Rc<Subwail>) -> Self {
        ExpandRightWail { left_wail: left, right_wail: right }
    }
}

fn add_offsets(a: &Offset, b: &Offset) -> Offset {
    a.shift(b.x, b.y, b.z)
}

impl Wail for ExpandRightWail {
    type Mdl = ExpandRightWailModel;

    fn update(&self, model: &mut ExpandRightWailModel, message: &WailIn) {
        match message {
            &WailIn::Offset(offset) => {
                let (left_base, right_base) = model.base_offsets;
                let (left_full, right_full) = (add_offsets(&left_base, &offset),
                                               add_offsets(&right_base, &offset));
                model.left_wailing.update(&WailIn::Offset(left_full));
                model.right_wailing.update(&WailIn::Offset(right_full));
            }
        }
    }
    fn view(&self, model: &ExpandRightWailModel) -> Vision<WailIn> {
        let left_vision = model.left_wailing.view();
        let right_vision = model.right_wailing.view();
        let mut vision = Vision::new() as Vision<WailIn>;
        vision.add_vision(left_vision, |_| None);
        vision.add_vision(right_vision, |_| None);
        vision
    }
    fn init(&self) -> ExpandRightWailModel {
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
        left_wailing.update(&WailIn::Offset(left_offset));
        right_wailing.update(&WailIn::Offset(right_offset));
        ExpandRightWailModel {
            frame: frame,
            left_wailing: left_wailing,
            right_wailing: right_wailing,
            base_offsets: (left_offset, right_offset)
        }
    }
    fn to_subwail(&self) -> Rc<Subwail> {
        Rc::new(ExpandRightSubwail { wail: self.clone(), wail_model: None }) as Rc<Subwail>
    }
}

#[derive(Debug)]
pub struct ExpandRightSubwail {
    wail: ExpandRightWail,
    wail_model: Option<ExpandRightWailModel>,
}

impl Subwail for ExpandRightSubwail {
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
                subwail: Box::new(ExpandRightSubwail {
                    wail: self.wail.clone(),
                    wail_model: Some(self.wail.init())
                }) as Box<Subwail>
            }
        }
    }
}

