use caravel::Caravel;
use caravel::ids_from_sigil;
use traveller::Traveller;
use vrcounter::sigil::Sigil;

pub struct SpectrumCaravel {
    starting_color_index: usize
}

impl Caravel for SpectrumCaravel {
    fn embark(&self) -> Traveller {
        let sigil = Sigil::of_fill();

        Traveller::Spectrum {
            ids: ids_from_sigil(&sigil),
            color_index: self.starting_color_index,
            sigil: sigil,
        }
    }
}

impl SpectrumCaravel {
    pub fn new(color_index: usize) -> Self {
        SpectrumCaravel {
            starting_color_index: color_index
        }
    }
}

