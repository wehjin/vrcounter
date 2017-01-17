use caravel::Caravel;
use caravel::ids_from_sigil;
use traveller::Traveller;
use vrcounter::sigil::Sigil;

pub struct SpectrumCaravel;

impl Caravel for SpectrumCaravel {
    fn embark(&self) -> Traveller {
        let sigil = Sigil::of_fill();
        Traveller::Spectrum {
            ids: ids_from_sigil(&sigil),
            color_index: 0,
            sigil: sigil,
        }
    }
}

impl SpectrumCaravel {
    pub fn new() -> Self {
        SpectrumCaravel {}
    }
}

