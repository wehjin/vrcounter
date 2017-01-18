use traveller::Traveller;
use std::marker::Send;
use std::marker::Sync;
use caravel::Caravel;

pub struct LambdaCaravel {
    on_embark: Box<Fn() -> Traveller + Send + Sync>,
}

impl Caravel for LambdaCaravel {
    fn embark(&self) -> Traveller {
        (self.on_embark)()
    }
}

impl LambdaCaravel {
    pub fn new<F>(on_embark: F) -> Self
        where F: Fn() -> Traveller + Send + Sync + 'static
    {
        LambdaCaravel { on_embark: Box::new(on_embark) }
    }
}