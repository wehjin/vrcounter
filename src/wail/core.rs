extern crate cage;
extern crate rand;

use hand::Hand;
use vision::Vision;
use cage::{Frame, Offset, Cage};
use std::rc::Rc;
use std::clone::Clone;

pub enum Biopt<A, B> {
    SomeA(A),
    SomeB(B),
}

#[derive(Clone)]
pub struct Wail2<Out> {
    on_summon: Rc<Fn() -> Box<Wailing<Out>>>
}

impl<Out> Wail2<Out> where Out: Clone + 'static {
    pub fn create(on_summon: Rc<Fn() -> Box<Wailing<Out>>>) -> Self {
        Wail2 { on_summon: on_summon }
    }
    pub fn summon(&self) -> Box<Wailing<Out>> {
        (*self.on_summon)()
    }
    pub fn add_touch(&self) -> Wail2<TouchMsg> {
        let base_wail = (*self).clone();
        Wail2::create(Rc::new(move || {
            let base_wailing = base_wail.summon();
            Box::new(TouchWailing {
                offset: Offset::default(),
                mist_id: rand::random::<u64>(),
                base_wailing: base_wailing,
            }) as Box<Wailing<TouchMsg>>
        }))
    }
    pub fn place_before<ExtOut>(&self, ext_wail: Wail2<ExtOut>) -> Wail2<Out> where ExtOut: Clone + 'static {
        let base_wail = (*self).clone();
        Wail2::create(Rc::new(move || {
            let ext_wailing = ext_wail.summon();
            let base_offset = Offset::from((0.0, 0.0, 0.10));
            let mut base_wailing = base_wail.summon();
            base_wailing.update(&WailingIn::Offset(base_offset.clone()));
            Box::new(BaseBeforeExtWailing {
                base_offset: base_offset,
                base_wailing: base_wailing,
                ext_wailing: ext_wailing,
            }) as Box<Wailing<Out>>
        }))
    }
}

pub fn color_wail(color: [f32; 4], frame: Frame) -> Wail2<()> {
    Wail2::create(Rc::new(move || {
        Box::new(ColorWailing {
            frame: frame, offset: Offset::default(),
            color: color, patch_id: rand::random::<u64>(),
        }) as Box<Wailing<()>>
    }))
}

pub struct BaseBeforeExtWailing<BaseOut, ExtOut> {
    base_offset: Offset,
    base_wailing: Box<Wailing<BaseOut>>,
    ext_wailing: Box<Wailing<ExtOut>>,
}

impl<BaseOut, ExtOut> Wailing<BaseOut> for BaseBeforeExtWailing<BaseOut, ExtOut> {
    fn size(&self) -> Frame {
        let base_frame = (*self.base_wailing).size();
        let ext_frame = (*self.ext_wailing).size();
        Frame::from((base_frame.w.max(ext_frame.w),
                     base_frame.h.max(ext_frame.h),
                     base_frame.d.max(ext_frame.d)))
    }

    fn view(&self) -> Vision<WailingIn> {
        let mut vision = Vision::new();
        let base_vision = (*self.base_wailing).view();
        vision.add_vision(base_vision, |x| Some(x));
        let ext_vision = (*self.ext_wailing).view();
        vision.add_vision(ext_vision, |_| None);
        vision
    }

    fn update(&mut self, message: &WailingIn) -> BaseOut {
        if let &WailingIn::Offset(offset) = message {
            (*self.ext_wailing).update(&WailingIn::Offset(offset.clone()));
            (*self.base_wailing).update(&WailingIn::Offset(offset.shift(self.base_offset.x,
                                                                        self.base_offset.y,
                                                                        self.base_offset.z)))
        } else {
            (*self.ext_wailing).update(message);
            (*self.base_wailing).update(message)
        }
    }
}

pub struct TouchWailing<BaseOut> {
    offset: Offset,
    mist_id: u64,
    base_wailing: Box<Wailing<BaseOut>>,
}

impl<BaseOut> Wailing<TouchMsg> for TouchWailing<BaseOut> {
    fn size(&self) -> Frame {
        (*self.base_wailing).size()
    }

    fn view(&self) -> Vision<WailingIn> {
        use mist::Mist;
        use common::Wish;
        let mut vision = Vision::new();
        vision.add_mist(Mist::new(self.mist_id, Cage::from((self.size(), self.offset))), |wish| {
            match wish {
                Wish::SenseHand(hand) => Some(WailingIn::Hand(hand)),
                _ => None,
            }
        });
        let base_vision = (*self.base_wailing).view();
        vision.add_vision(base_vision, |_| { None });
        vision
    }

    fn update(&mut self, message: &WailingIn) -> TouchMsg {
        match message {
            &WailingIn::Offset(offset) => {
                self.offset = offset;
                (*self.base_wailing).update(&WailingIn::Offset(offset));
                TouchMsg::None
            },
            &WailingIn::Hand(_) => {
                TouchMsg::TouchMove
            },
        }
    }
}

pub struct ColorWailing {
    frame: Frame,
    offset: Offset,
    color: [f32; 4],
    patch_id: u64,
}

impl Wailing<()> for ColorWailing {
    fn size(&self) -> Frame {
        self.frame
    }

    fn view(&self) -> Vision<WailingIn> {
        use patch::{Patch, Sigil};
        use cage::Cage;
        let mut vision = Vision::new();
        let patch = Patch::from_cage(&Cage::from((self.frame, self.offset)), self.color, Sigil::Fill, self.patch_id);
        vision.add_patch(patch);
        vision
    }

    fn update(&mut self, message: &WailingIn) -> () {
        match message {
            &WailingIn::Hand(_) => (),
            &WailingIn::Offset(offset) => {
                self.offset = offset;
            }
        };
        ()
    }
}

pub trait Wailing<MsgOut> {
    fn size(&self) -> Frame;
    fn view(&self) -> Vision<WailingIn>;
    fn update(&mut self, message: &WailingIn) -> MsgOut;
}

#[derive(Clone, Debug)]
pub enum TouchMsg {
    None,
    TouchMove
}

#[derive(Copy, Clone, Debug)]
pub enum WailingIn {
    Offset(Offset),
    Hand(Hand),
}
