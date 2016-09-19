extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset};
use std::fmt::Debug;
use super::expand_right::*;
use std::marker::Sized;

#[derive(Copy, Clone, Debug)]
pub enum WailIn {
    Offset(Offset),
}

#[derive(Copy, Clone, Debug)]
pub enum WailOut {
    Frame(Frame),
}

pub trait Wail: Debug {
    type Mdl: Wailing + 'static;

    fn expand_right<TRight: Wail>(self, right_wail: TRight) -> ExpandRightWail<Self, TRight> where Self: Sized {
        ExpandRightWail::new(self, right_wail)
    }
    fn update(&self, model: &mut Self::Mdl, message: &WailIn);
    fn view(&self, model: &Self::Mdl) -> Vision<WailIn>;
    fn summon(self) -> Self::Mdl;
}

pub trait Wailing: Debug {
    fn report_frame(&self) -> Frame;
    fn update(&mut self, message: &WailIn);
    fn view(&self) -> Vision<WailIn>;
}
