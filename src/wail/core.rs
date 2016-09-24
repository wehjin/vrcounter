extern crate cage;
extern crate rand;

use hand::Hand;
use vision::Vision;
use cage::{Frame, Offset};
use std::fmt::Debug;
use std::marker::PhantomData;
use super::expand_right::*;
use super::in_front_of::*;
use super::enable_hand::*;

use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum WailerIn {
    Offset(Offset),
    Hand(Hand),
}

#[derive(Copy, Clone, Debug)]
pub enum WailerOut {
    Frame(Frame),
}

pub enum Biopt<A, B> {
    SomeA(A),
    SomeB(B),
}

pub trait Wailer<O>: Clone where O: Clone + Debug + 'static {
    type Mdl: 'static;

    fn enable_hand<ONext, F>(&self, adapt: F) -> EnableHandWailer<O, ONext>
        where ONext: Clone + Debug + 'static, F: 'static + Fn(Biopt<O, EnableHandOut>) -> ONext
    {
        EnableHandWailer::new(self.to_subwail(), adapt)
    }
    fn in_front_of<A, ONext, AWailer, F>(&self, far_wail: AWailer, adapt: F) -> InFrontOfWailer<O, A, ONext>
        where A: Clone + Debug + 'static, ONext: Clone + Debug + 'static, AWailer: Wailer<A>, F: 'static + Fn(Biopt<O, A>) -> ONext
    {
        InFrontOfWailer::new(self.to_subwail(), far_wail.to_subwail(), adapt)
    }
    fn expand_right<B, ONext, BWailer, F>(&self, right_wail: BWailer, adapt: F) -> ExpandRightWailer<O, B, ONext>
        where B: Clone + Debug + 'static, ONext: Clone + Debug + 'static, BWailer: Wailer<B>, F: 'static + Fn(Biopt<O, B>) -> ONext
    {
        ExpandRightWailer::new(self.to_subwail(), right_wail.to_subwail(), adapt)
    }
    fn report(&self, model: &Self::Mdl) -> Vec<O>;
    fn update(&self, model: &mut Self::Mdl, message: &WailerIn);
    fn view(&self, model: &Self::Mdl) -> Vision<WailerIn>;
    fn init(&self) -> Self::Mdl;

    fn to_subwail(&self) -> Rc<Subwailer<O>>;
    fn summon(&self) -> Wailing<O> {
        self.to_subwail().as_ref().summon()
    }
}

// Do not add Clone. We need to box this trait.
pub trait Subwailer<E> {
    fn report(&self) -> Vec<E>;
    fn report_frame(&self) -> Frame;
    fn update(&mut self, message: &WailerIn);
    fn view(&self) -> Vision<WailerIn>;
    fn summon(&self) -> Wailing<E>;
}

pub struct Wailing<E> {
    pub subwail: Box<Subwailer<E>>
}


impl<E> Wailing<E> where E: 'static {
    pub fn empty() -> Self {
        let subwailer = EmptySubwailer::new() as EmptySubwailer<E>;
        subwailer.summon()
    }

    pub fn report(&self) -> Vec<E> {
        self.subwail.as_ref().report()
    }
    pub fn report_frame(&self) -> Frame {
        self.subwail.as_ref().report_frame()
    }
    pub fn update(&mut self, message: &WailerIn) {
        self.subwail.as_mut().update(message);
    }
    pub fn view(&self) -> Vision<WailerIn> {
        self.subwail.as_ref().view()
    }
}

pub struct EmptySubwailer<E> {
    report_type: PhantomData<E>
}

impl<E> EmptySubwailer<E> {
    fn new() -> Self {
        EmptySubwailer { report_type: PhantomData }
    }
}

impl<E> Subwailer<E> for EmptySubwailer<E> where E: 'static {
    fn report(&self) -> Vec<E> {
        vec![]
    }

    fn report_frame(&self) -> Frame {
        Frame::default()
    }

    fn update(&mut self, _: &WailerIn) {
        // Do nothing
    }

    fn view(&self) -> Vision<WailerIn> {
        Vision::default()
    }

    fn summon(&self) -> Wailing<E> {
        Wailing { subwail: Box::new(EmptySubwailer::new()) as Box<Subwailer<E>> }
    }
}
