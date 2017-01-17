use caravel::Caravel;
use traveller::Traveller;

pub struct DockLeftCaravel<LeftC: Caravel, RightC: Caravel>
{
    left_units: f32,
    left_caravel: LeftC,
    right_caravel: RightC,
}

impl<LeftC, RightC> Caravel for DockLeftCaravel<LeftC, RightC>
where LeftC: Caravel, RightC: Caravel
{
    fn embark(&self) -> Traveller {
        Traveller::DockLeft {
            left_units: self.left_units,
            left_traveller: Box::new(self.left_caravel.embark()),
            right_traveller: Box::new(self.right_caravel.embark()),
        }
    }
}

impl<LeftC, RightC> DockLeftCaravel<LeftC, RightC>
where LeftC: Caravel, RightC: Caravel
{
    pub fn new(left_units: f32, left_caravel: LeftC, right_caravel: RightC) -> Self {
        DockLeftCaravel {
            left_units: left_units,
            left_caravel: left_caravel,
            right_caravel: right_caravel,
        }
    }
}



