use std::boxed::Box;
use std::collections::HashMap;
use patch::Patch;
use mist::Mist;

pub enum VisionMessage {
    Tick,
}

pub struct Vision<Msg> {
    adapt_message: Box<Fn(VisionMessage) -> Msg>,
    patches: HashMap<u64, Patch>,
    mists: HashMap<u64, Mist>,
}

pub enum Report<Mod, Out> {
    Model(Mod),
    Outcome(Out),
    Error,
}

pub struct Roar<Mod, Msg, Out> {
    init: Box<Fn() -> Mod>,
    update: Box<Fn(Mod) -> Report<Mod, Out>>,
    view: Box<Fn(&Mod) -> Vision<Msg>>,
}

impl<Mod, Msg, Out> Roar<Mod, Msg, Out> {
    pub fn create(init: Box<Fn() -> Mod>, update: Box<Fn(Mod) -> Report<Mod, Out>>, view: Box<Fn(&Mod) -> Vision<Msg>>) -> Self {
        Roar { init: init, update: update, view: view }
    }
}

pub struct Summoner;

impl Summoner {
    pub fn present<Msg, SubMod, SubMsg, SubOut>(roar: &Roar<SubMod, SubMsg, SubOut>, adapt_report: Box<Fn(SubOut) -> Msg>) -> u64 {
        0u64
    }
}
