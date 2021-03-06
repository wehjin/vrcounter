extern crate vrcounter;
extern crate cage;
extern crate rand;

use vrcounter::*;
use vrcounter::color::*;
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
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
    wailing: Rc<RefCell<Box<Wailing<TouchMsg>>>>,
}

impl Star for App {
    type Mdl = Model;
    type Msg = Msg;
    type Out = Out;

    fn init(&self) -> Model {
        let frame = Frame::from((0.20, 0.20, 0.20));
        let wail = color_wail(CYAN, frame).add_touch()
                                          .place_before(color_wail(RED, Frame::default()));
        let wailing = wail.summon();
        Model {
            wailing: Rc::new(RefCell::new(wailing))
        }
    }

    fn update(&self, model: &Model, msg: &Msg) -> Model {
        match msg {
            &Msg::SendToWailing(wailing_in) => {
                {
                    let mut wailing = model.wailing.as_ref().borrow_mut();
                    let out = wailing.update(&wailing_in);
                    match out {
                        TouchMsg::TouchMove => {
                            println!("Touch move!");
                        }
                        _ => ()
                    }
                }
                model.clone()
            }
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
}


fn main() {
    let star_builder = Arc::new(move || App);
    vrcounter::start(star_builder)
}
