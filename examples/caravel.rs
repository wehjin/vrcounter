use traveller::Traveller;
use traveller::color::ColorTraveller;

pub trait Caravel<T: Traveller> {
    fn embark(&self) -> T;
}

pub struct ColorCaravel {
    color: [f32; 4],
}

impl ColorCaravel {
    pub fn new(color: [f32; 4]) -> Self {
        ColorCaravel { color: color }
    }
}

impl Caravel<ColorTraveller> for ColorCaravel {
    fn embark(&self) -> ColorTraveller {
        ColorTraveller::new(self.color)
    }
}
