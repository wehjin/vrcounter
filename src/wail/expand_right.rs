extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use super::*;

#[derive(Clone, Debug)]
pub struct ExpandRightWail<TLeft, TRight>
    where TLeft: Wail, TRight: Wail
{
    left_wail: TLeft,
    right_wail: TRight,
}

impl<TLeft, TRight> ExpandRightWail<TLeft, TRight>
where TLeft: Wail, TRight: Wail
{
    pub fn new(left: TLeft, right: TRight) -> Self {
        ExpandRightWail { left_wail: left, right_wail: right }
    }
}

impl<TLeft, TRight> Wail for ExpandRightWail<TLeft, TRight>
where TLeft: Wail + 'static + Clone, TRight: Wail + 'static + Clone
{
    type Mdl = ExpandRightWailing<TLeft, TRight>;

    fn update(&self, _: &mut ExpandRightWailing<TLeft, TRight>, _: &WailIn) {
        // TODO Implement WailIn::Offset
    }

    fn view(&self, model: &ExpandRightWailing<TLeft, TRight>) -> Vision<WailIn> {
        let left_vision = model.left_wailing.as_ref().view();
        let right_vision = model.right_wailing.as_ref().view();
        let mut vision = Vision::new() as Vision<WailIn>;
        vision.add_vision(left_vision, |_| None);
        vision.add_vision(right_vision, |_| None);
        vision
    }

    fn summon(self) -> ExpandRightWailing<TLeft, TRight> {
        let mut left_wailing = self.left_wail.clone().summon();
        let mut right_wailing = self.right_wail.clone().summon();
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
        ExpandRightWailing {
            expand_right_wail: self,
            frame: frame,
            left_wailing: Box::new(left_wailing) as Box<Wailing>,
            right_wailing: Box::new(right_wailing) as Box<Wailing>,
        }
    }
}

#[derive(Debug)]
pub struct ExpandRightWailing<TLeft, TRight>
    where TLeft: Wail + 'static, TRight: Wail + 'static
{
    expand_right_wail: ExpandRightWail<TLeft, TRight>,
    frame: Frame,
    left_wailing: Box<Wailing>,
    right_wailing: Box<Wailing>,
}

impl<TLeft, TRight> Wailing for ExpandRightWailing<TLeft, TRight>
where TLeft: Wail + 'static + Clone, TRight: Wail + 'static + Clone
{
    fn update(&mut self, message: &WailIn) {
        let expand_right_wail = self.expand_right_wail.clone();
        expand_right_wail.update(self, message);
    }
    fn view(&self) -> Vision<WailIn> {
        self.expand_right_wail.view(self)
    }
    fn report_frame(&self) -> Frame {
        self.frame.clone()
    }
}
