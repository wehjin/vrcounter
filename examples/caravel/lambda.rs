use traveller::Traveller;
use std::sync::Arc;
use std::marker::Send;
use std::marker::Sync;
use caravel::Caravel;

pub struct LambdaCaravel {
    on_embark: Arc<Fn() -> Traveller + Send + Sync>,
}

impl Caravel for LambdaCaravel {
    fn embark(&self) -> Traveller {
        (self.on_embark)()
    }
}

impl LambdaCaravel {
    pub fn new<F>(on_embark: Arc<F>) -> Self
        where F: Fn() -> Traveller + Send + Sync + 'static
    {
        LambdaCaravel { on_embark: on_embark }
    }
}