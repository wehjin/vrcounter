use caravel::Caravel;
use traveller::Traveller;
use std::boxed::Box;

pub struct ProxyCaravel {
    delegate: Box<Caravel>
}

impl Caravel for ProxyCaravel {
    fn embark(&self) -> Traveller {
        self.delegate.embark()
    }
}

impl ProxyCaravel {
    pub fn new<C>(delegate: C) -> Self where C: Caravel + 'static {
        ProxyCaravel { delegate: Box::new(delegate) }
    }
}
