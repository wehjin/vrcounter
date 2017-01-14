use caravel::Caravel;
use traveller::color::ColorTraveller;

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
