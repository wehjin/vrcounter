extern crate cage;
extern crate rand;

use hand::Hand;
use vision::Vision;
use cage::{Frame, Offset, Cage};
use std::fmt::Debug;
use std::rc::Rc;

pub enum Biopt<A, B> {
    SomeA(A),
    SomeB(B),
}

pub trait Wail<MsgOut> where MsgOut: Clone + Debug + 'static {
    //fn with_touch(&self) -> TouchWail;
    //fn in_front_of<T: Wail<_>>(&self) -> InFrontOfWail;
    fn summon(&self) -> Box<Wailing<MsgOut>>;
}

#[derive(Copy, Clone, Debug)]
pub enum WailingIn {
    Offset(Offset),
    Hand(Hand),
}

pub trait Wailing<MsgOut> {
    fn size(&self) -> Frame;
    fn view(&self) -> Vision<WailingIn>;
    fn update(&mut self, message: &WailingIn) -> MsgOut;
}

pub struct ColorWail {
    pub frame: Frame,
    pub color: [f32; 4],
}

impl Wail<()> for ColorWail {
    fn summon(&self) -> Box<Wailing<()>> {
        Box::new(ColorWailing {
            frame: self.frame, offset: Offset::default(),
            color: self.color, patch_id: rand::random::<u64>(),
        }) as Box<Wailing<()>>
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
