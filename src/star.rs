use vision::Vision;
use common::Wish;


pub trait Star: Clone {
    type Mdl: Clone;
    type Msg: Clone;
    type Out: Clone;

    fn init(&self) -> (Self::Mdl, Vec<Wish>);
    fn update(&self, Self::Msg, &Self::Mdl) -> (Option<Self::Mdl>, Vec<Wish>, Vec<Self::Out>);
    fn view(&self, &Self::Mdl) -> Vision<Self::Msg>;
}
