extern crate cage;
extern crate rand;

use vision::Vision;
use cage::{Frame, Offset, Cage};
use patch::{Sigil, Patch};
use std::fmt::Debug;

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
    fn update(&self, message: &WailIn) -> Self;
    fn view(&self) -> Vision<WailIn>;
}

#[derive(Clone, Debug)]
pub struct ExpandRightWail<TLeft, TRight>
    where TLeft: Wail, TRight: Wail
{
    left_wail: TLeft,
    right_wail: TRight,
}

impl<TLeft, TRight> ExpandRightWail<TLeft, TRight>
where TLeft: Wail, TRight: Wail
{
    pub fn new(left: TLeft, right: TRight) -> Self {
        ExpandRightWail { left_wail: left, right_wail: right }
    }
}

impl<TLeft, TRight> Wail for ExpandRightWail<TLeft, TRight>
where TLeft: Wail, TRight: Wail
{
    type Mdl = ExpandRightWailing<TLeft, TRight>;

    fn update(&self, model: &ExpandRightWailing<TLeft, TRight>, _: &WailIn) -> ExpandRightWailing<TLeft, TRight> {
        // TODO Implement!
        (*model).clone()
    }

    fn view(&self, model: &ExpandRightWailing<TLeft, TRight>) -> Vision<WailIn> {
        let left_vision = self.left_wail.view(&model.left_wailing);
        let right_vision = self.right_wail.view(&model.right_wailing);
        let mut vision = Vision::new() as Vision<WailIn>;
        vision.add_vision(left_vision, |_| None);
        vision.add_vision(right_vision, |_| None);
        vision
    }

    fn summon(self) -> ExpandRightWailing<TLeft, TRight> {
        let left_wailing = self.left_wail.clone().summon();
        let right_wailing = self.right_wail
            .clone()
            .summon()
            .update(&WailIn::Offset(Offset::from((0.25, 0.0, 0.0))));
        ExpandRightWailing { expand_right_wail: self, left_wailing: left_wailing, right_wailing: right_wailing }
    }
}

#[derive(Clone, Debug)]
pub struct ExpandRightWailing<TLeft, TRight>
    where TLeft: Wail + Clone, TRight: Wail + Clone
{
    expand_right_wail: ExpandRightWail<TLeft, TRight>,
    left_wailing: TLeft::Mdl,
    right_wailing: TRight::Mdl,
}

impl<TLeft, TRight> Wailing for ExpandRightWailing<TLeft, TRight>
where TLeft: Wail, TRight: Wail
{
    fn update(&self, message: &WailIn) -> Self {
        self.expand_right_wail.update(self, message)
    }
    fn view(&self) -> Vision<WailIn> {
        self.expand_right_wail.view(self)
    }
}


#[derive(Clone, Debug)]
pub struct LeafWail {
    color: [f32; 4],
    frame: Frame,
}

#[derive(Clone, Debug)]
pub struct LeafWailing {
    offset: Offset,
    patch_id: u64,
    leaf_wail: LeafWail,
}

impl Wailing for LeafWailing {
    fn update(&self, message: &WailIn) -> Self {
        self.leaf_wail.update(self, message)
    }
    fn view(&self) -> Vision<WailIn> {
        self.leaf_wail.view(self)
    }
}

impl LeafWail {
    pub fn new(color: [f32; 4], frame: Frame) -> Self {
        LeafWail { color: color, frame: frame }
    }
}

impl Wail for LeafWail {
    type Mdl = LeafWailing;

    fn update(&self, model: &LeafWailing, message: &WailIn) -> LeafWailing {
        let mut new_model = (*model).clone();
        match message {
            &WailIn::Offset(offset) => {
                new_model.offset = offset;
            }
        }
        new_model
    }
    fn view(&self, model: &LeafWailing) -> Vision<WailIn> {
        let cage = Cage::from((self.frame, model.offset));
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, model.patch_id);
        let mut vision = Vision::new();
        vision.add_patch(patch);
        vision
    }
    fn summon(self) -> LeafWailing {
        let patch_id = rand::random::<u64>();
        let offset = Offset::default();
        LeafWailing { offset: offset, patch_id: patch_id, leaf_wail: self }
    }
}
