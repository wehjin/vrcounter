use vision::Vision;
use report::Well;


pub trait Star: Clone {
    type Mdl: Clone;
    type Msg: Clone;
    type Out: Clone;

    fn init(&self) -> Self::Mdl;
    fn view(&self, &Self::Mdl) -> Vision<Self::Msg>;
    fn update<T>(&self, &Self::Mdl, Self::Msg, &mut Well<Self::Out, T>) -> Option<Self::Mdl>;
}
