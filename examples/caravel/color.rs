use caravel::Caravel;
use caravel::top_dock::TopDockCaravel;
use traveller::Traveller;
use traveller::color::ColorTraveller;

pub struct ColorCaravel {
    color: [f32; 4],
}

impl Caravel<ColorTraveller> for ColorCaravel {
    fn embark(&self) -> ColorTraveller {
        ColorTraveller::new(self.color)
    }
}

impl ColorCaravel {
    pub fn new(color: [f32; 4]) -> Self {
        ColorCaravel { color: color }
    }

    pub fn dock_top<TopT, TopC>(self, top_units: f32, top_caravel: TopC)
                                -> TopDockCaravel<ColorTraveller, Self, TopT, TopC>
        where TopT: Traveller, TopC: Caravel<TopT>
    {
        TopDockCaravel::new(top_units, self, top_caravel)
    }
}

