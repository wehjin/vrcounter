use vision::Vision;
use report::Well;


pub trait Star: Clone {
    type Mdl: Clone;
    type Msg: Clone;
    type Out: Clone;

    fn init(&self) -> Self::Mdl;
    fn view(&self, &Self::Mdl) -> Vision<Self::Msg>;
    fn update(&self, &Self::Mdl, Self::Msg) -> Option<Self::Mdl>;
    fn report<T>(&self, &Self::Mdl, &mut Well<Self::Out, T>) {}
}
