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
where TLeft: Wail, TRight: Wail
{
    type Mdl = ExpandRightWailing<TLeft, TRight>;

    fn update(&self, model: &ExpandRightWailing<TLeft, TRight>, _: &WailIn) -> ExpandRightWailing<TLeft, TRight> {
        // TODO Implement!
        (*model).clone()
    }

    fn view(&self, model: &ExpandRightWailing<TLeft, TRight>) -> Vision<WailIn> {
        let left_vision = self.left_wail.view(&model.left_wailing);
        let right_vision = self.right_wail.view(&model.right_wailing);
        let mut vision = Vision::new() as Vision<WailIn>;
        vision.add_vision(left_vision, |_| None);
        vision.add_vision(right_vision, |_| None);
        vision
    }

    fn summon(self) -> ExpandRightWailing<TLeft, TRight> {
        let left_wailing = self.left_wail.clone().summon();
        let left_frame = left_wailing.report_frame();
        let right_wailing = self.right_wail.clone().summon();
        let right_frame = right_wailing.report_frame();
        let frame = Frame::from((left_frame.w + right_frame.w,
                                 left_frame.h.max(right_frame.h),
                                 left_frame.d.max(right_frame.d)));
        // TODO Deal with mis-matched y and z in offsets.
        let right_offset = Offset::from((frame.w / 2.0 - right_frame.w / 2.0, 0.0, 0.0));
        let left_offset = Offset::from((-frame.w / 2.0 + left_frame.w / 2.0, 0.0, 0.0));
        ExpandRightWailing {
            expand_right_wail: self,
            frame: frame,
            left_wailing: left_wailing.update(&WailIn::Offset(left_offset)),
            right_wailing: right_wailing.update(&WailIn::Offset(right_offset))
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExpandRightWailing<TLeft, TRight>
    where TLeft: Wail + Clone, TRight: Wail + Clone
{
    expand_right_wail: ExpandRightWail<TLeft, TRight>,
    frame: Frame,
    left_wailing: TLeft::Mdl,
    right_wailing: TRight::Mdl,
}

impl<TLeft, TRight> Wailing for ExpandRightWailing<TLeft, TRight>
where TLeft: Wail, TRight: Wail
{
    fn update(&self, message: &WailIn) -> Self {
        self.expand_right_wail.update(self, message)
    }
    fn view(&self) -> Vision<WailIn> {
        self.expand_right_wail.view(self)
    }
    fn report_frame(&self) -> Frame {
        self.frame.clone()
    }
}
