use caravel::Caravel;
use traveller::spectrum::SpectrumTraveller;

pub struct SpectrumCaravel;

impl Caravel<SpectrumTraveller> for SpectrumCaravel {
    fn embark(&self) -> SpectrumTraveller {
        SpectrumTraveller::new()
    }
}

impl SpectrumCaravel {
    pub fn new() -> Self {
        SpectrumCaravel {}
    }
}

