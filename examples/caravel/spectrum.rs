use caravel::Caravel;
use caravel::ids_from_sigil;
use traveller::Traveller2;
use vrcounter::sigil::Sigil;

pub struct SpectrumCaravel;

impl Caravel for SpectrumCaravel {
    fn embark(&self) -> Traveller2 {
        let sigil = Sigil::of_fill();
        Traveller2::Spectrum {
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

