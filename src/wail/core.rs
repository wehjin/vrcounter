extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use std::fmt::Debug;
use super::expand_right::*;

#[derive(Copy, Clone, Debug)]
pub enum WailIn {
    Offset(Offset),
}

#[derive(Copy, Clone, Debug)]
pub enum WailOut {
    Frame(Frame),
}

pub trait Wail: Clone + Debug {
    type Mdl: Wailing;
    fn expand_right<TRight: Wail>(self, right_wail: TRight) -> ExpandRightWail<Self, TRight> {
        ExpandRightWail::new(self, right_wail)
    }
    fn update(&self, model: &Self::Mdl, message: &WailIn) -> Self::Mdl;
    fn view(&self, model: &Self::Mdl) -> Vision<WailIn>;
    fn summon(self) -> Self::Mdl;
}

pub trait Wailing: Clone + Debug {
    fn report_frame(&self) -> Frame;
    fn update(&self, message: &WailIn) -> Self;
    fn view(&self) -> Vision<WailIn>;
}
