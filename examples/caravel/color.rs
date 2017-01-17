use caravel::Caravel;
use caravel::ids_from_sigil;
use traveller::Traveller;
use vrcounter::Sigil;

pub struct ColorCaravel {
    color: [f32; 4],
    sigil: Sigil,
}

impl Caravel for ColorCaravel {
    fn embark(&self) -> Traveller {
        Traveller::Color {
            ids: ids_from_sigil(&self.sigil),
            color: self.color,
            sigil: self.sigil.clone()
        }
    }
}

impl ColorCaravel {
    pub fn new(sigil: Sigil, color: [f32; 4]) -> Self {
        ColorCaravel { color: color, sigil: sigil }
    }
}

