use caravel::Caravel;
use traveller::Traveller;

pub struct DockTopCaravel<BottomC: Caravel, TopC: Caravel>
{
    top_units: f32,
    bottom_caravel: BottomC,
    top_caravel: TopC,
}

impl<BottomC, TopC> Caravel for DockTopCaravel<BottomC, TopC>
where BottomC: Caravel, TopC: Caravel
{
    fn embark(&self) -> Traveller {
        Traveller::DockTop {
            top_units: self.top_units,
            bottom_traveller: Box::new(self.bottom_caravel.embark()),
            top_traveller: Box::new(self.top_caravel.embark())
        }
    }
}

impl<BottomC, TopC> DockTopCaravel<BottomC, TopC>
where BottomC: Caravel, TopC: Caravel
{
    pub fn new(top_units: f32, bottom_caravel: BottomC, top_caravel: TopC) -> Self {
        DockTopCaravel {
            top_units: top_units,
            bottom_caravel: bottom_caravel,
            top_caravel: top_caravel,
        }
    }
}



