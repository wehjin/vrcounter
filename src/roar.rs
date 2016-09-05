use std::boxed::Box;
use std::rc::Rc;
use std::collections::HashMap;
use patch::Patch;
use mist::Mist;
use common::IdSource;
use std::any::Any;

pub struct Roar<Mod, Msg, Out> {
    init: Rc<Fn() -> Mod>,
    update: Rc<Fn(Mod) -> Report<Mod, Out>>,
    view: Rc<Fn(&Mod) -> Vision<Msg>>,
}

impl<Mod, Msg, Out> Roar<Mod, Msg, Out> {
    pub fn create(init: Rc<Fn() -> Mod>, update: Rc<Fn(Mod) -> Report<Mod, Out>>, view: Rc<Fn(&Mod) -> Vision<Msg>>) -> Self {
        Roar { init: init, update: update, view: view }
    }
}

pub enum DemonResult {
    Remain,
    Remove,
}

pub trait Demon {
    fn id(&self) -> u64;
    fn send(&self, vision_message: VisionMessage) -> DemonResult;
}

pub struct Demonoid<Mod, Msg, Out> {
    id: u64,
    update: Rc<Fn(Mod) -> Report<Mod, Out>>,
    view: Rc<Fn(&Mod) -> Vision<Msg>>,
}

impl<Mod, Msg, Out> Demon for Demonoid<Mod, Msg, Out> {
    fn id(&self) -> u64 {
        self.id
    }

    fn send(&self, vision_message: VisionMessage) -> DemonResult {
        DemonResult::Remain
    }
}


pub struct Summoner {
    demons: HashMap<u64, Box<Demon>>,
}

impl Summoner {
    pub fn summon<Msg, SubMod: 'static, SubMsg: 'static, SubOut: 'static>(&mut self, id_source: &mut IdSource, roar: &Roar<SubMod, SubMsg, SubOut>, adapt_report: Box<Fn(SubOut) -> Msg>) -> u64 {
        let model = ((*roar).init)();
        let vision = ((*roar).view)(&model);
        let id = id_source.next_id();
        let demon = Demonoid { id: id, update: roar.update.clone(), view: roar.view.clone() };
        self.demons.insert(id, Box::new(demon));
        id
    }
}

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
