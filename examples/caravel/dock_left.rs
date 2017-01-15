use caravel::Caravel;
use traveller::Traveller;
use traveller::dock_left::DockLeftTraveller;
use std::marker::PhantomData;

pub struct DockLeftCaravel<LeftT: Traveller, LeftC: Caravel<LeftT>, RightT: Traveller, RightC: Caravel<RightT>>
{
    left_units: f32,
    left_caravel: LeftC,
    right_caravel: RightC,
    left_phantom: PhantomData<LeftT>,
    right_phantom: PhantomData<RightT>,
}

impl<LeftT, LeftC, RightT, RightC> Caravel<DockLeftTraveller<LeftT, RightT>>
for DockLeftCaravel<LeftT, LeftC, RightT, RightC>
where LeftT: Traveller,
      LeftC: Caravel<LeftT>,
      RightT: Traveller,
      RightC: Caravel<RightT>
{
    fn embark(&self) -> DockLeftTraveller<LeftT, RightT> {
        DockLeftTraveller::new(self.left_units,
                               self.left_caravel.embark(),
                               self.right_caravel.embark())
    }
}

impl<LeftT, LeftC, RightT, RightC> DockLeftCaravel<LeftT, LeftC, RightT, RightC>
where LeftT: Traveller,
      LeftC: Caravel<LeftT>,
      RightT: Traveller,
      RightC: Caravel<RightT>
{
    pub fn new(left_units: f32, left_caravel: LeftC, right_caravel: RightC) -> Self {
        DockLeftCaravel {
            left_units: left_units,
            left_caravel: left_caravel,
            right_caravel: right_caravel,
            left_phantom: PhantomData,
            right_phantom: PhantomData,
        }
    }
}



