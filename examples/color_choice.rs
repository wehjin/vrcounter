extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::*;
use vrcounter::color::*;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use rand::random;
use cage::{Frame};

#[derive(Clone, Debug)]
enum Msg {
    SendToWailing(WailingIn)
}

#[derive(Clone, Debug)]
struct Out;

#[derive(Clone, Debug)]
struct App;

#[derive(Clone)]
struct Model {
    patch_id: u64,
    beat_id: u64,
    wailing: Rc<RefCell<Box<Wailing<()>>>>,
}

impl Star for App {
    type Mdl = Model;
    type Msg = Msg;
    type Out = Out;

    fn init(&self) -> Model {
        let frame = Frame::from((0.20, 0.20, 0.20));
        let wail = ColorWail { frame: frame, color: CYAN };
        let wailing = wail.summon();
        Model {
            patch_id: random::<u64>(),
            beat_id: random::<u64>(),
            wailing: Rc::new(RefCell::new(wailing))
        }
    }

    fn view(&self, model: &Model) -> Vision<Msg> {
        let mut vision = Vision::new();
        let wailing_mut = model.wailing.as_ref().borrow_mut();
        let wail_vision = wailing_mut.view();
        vision.add_vision(wail_vision, |wailing_msg| {
            match wailing_msg {
                WailingIn::Hand(hand) => Some(Msg::SendToWailing(WailingIn::Hand(hand))),
                _ => None
            }
        });
        vision
    }

    fn update(&self, model: &Model, msg: &Msg) -> Model {
        match msg {
            &Msg::SendToWailing(wailing_in) => {
                {
                    let mut wailing_mut = model.wailing.as_ref().borrow_mut();
                    wailing_mut.update(&wailing_in);
                }
                model.clone()
            }
        }
    }
}


fn main() {
    let star_builder = Arc::new(move || App);
    vrcounter::start(star_builder)
}
