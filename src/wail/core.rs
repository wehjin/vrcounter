extern crate cage;
extern crate rand;

use hand::Hand;
use vision::Vision;
use cage::{Frame, Offset};

pub enum Biopt<A, B> {
    SomeA(A),
    SomeB(B),
}

pub struct Wail2<Out> {
    on_summon: Box<Fn() -> Box<Wailing<Out>>>
}

impl<Out> Wail2<Out> {
    pub fn create(on_summon: Box<Fn() -> Box<Wailing<Out>>>) -> Self {
        Wail2 { on_summon: on_summon }
    }
    pub fn summon(&self) -> Box<Wailing<Out>> {
        (*self.on_summon)()
    }
}

pub trait Wailing<MsgOut> {
    fn size(&self) -> Frame;
    fn view(&self) -> Vision<WailingIn>;
    fn update(&mut self, message: &WailingIn) -> MsgOut;
}

pub fn color_wail(color: [f32; 4], frame: Frame) -> Wail2<()> {
    Wail2::create(Box::new(move || {
        Box::new(ColorWailing {
            frame: frame, offset: Offset::default(),
            color: color, patch_id: rand::random::<u64>(),
        }) as Box<Wailing<()>>
    }))
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

#[derive(Copy, Clone, Debug)]
pub enum WailingIn {
    Offset(Offset),
    Hand(Hand),
}
