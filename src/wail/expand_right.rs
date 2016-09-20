extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use super::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct ExpandRightWailerModel
{
    frame: Frame,
    left_wailing: Wailing,
    right_wailing: Wailing,
    base_offsets: (Offset, Offset)
}

#[derive(Clone, Debug)]
pub struct ExpandRightWailer {
    left_wail: Rc<Subwailer>,
    right_wail: Rc<Subwailer>,
}

impl ExpandRightWailer {
    pub fn new(left: Rc<Subwailer>, right: Rc<Subwailer>) -> Self {
        ExpandRightWailer { left_wail: left, right_wail: right }
    }
}

fn add_offsets(a: &Offset, b: &Offset) -> Offset {
    a.shift(b.x, b.y, b.z)
}

impl Wailer for ExpandRightWailer {
    type Mdl = ExpandRightWailerModel;

    fn update(&self, model: &mut ExpandRightWailerModel, message: &WailerIn) {
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
    fn view(&self, model: &ExpandRightWailerModel) -> Vision<WailerIn> {
        let left_vision = model.left_wailing.view();
        let right_vision = model.right_wailing.view();
        let mut vision = Vision::new() as Vision<WailerIn>;
        vision.add_vision(left_vision, |_| None);
        vision.add_vision(right_vision, |_| None);
        vision
    }
    fn init(&self) -> ExpandRightWailerModel {
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
    fn to_subwail(&self) -> Rc<Subwailer> {
        Rc::new(ExpandRightSubwailer { wail: self.clone(), wail_model: None }) as Rc<Subwailer>
    }
}

subwail!(ExpandRightSubwailer, ExpandRightWailer, ExpandRightWailerModel);

