use caravel::Caravel;
use traveller::color::ColorTraveller;
use vrcounter::Sigil;

pub struct ColorCaravel {
    color: [f32; 4],
    sigil: Sigil,
}

impl Caravel<ColorTraveller> for ColorCaravel {
    fn embark(&self) -> ColorTraveller {
        ColorTraveller::new(self.color, self.sigil.clone())
    }
}

impl ColorCaravel {
    pub fn new(color: [f32; 4], sigil: Sigil) -> Self {
        ColorCaravel { color: color, sigil: sigil }
    }
}

