use caravel::Caravel;
use traveller::Traveller;
use traveller::dock_top::DockTopTraveller;
use std::marker::PhantomData;

pub struct DockTopCaravel<BottomT: Traveller, BottomC: Caravel<BottomT>, TopT: Traveller, TopC: Caravel<TopT>>
{
    top_units: f32,
    bottom_caravel: BottomC,
    top_caravel: TopC,
    top_phantom: PhantomData<BottomT>,
    bottom_phantom: PhantomData<TopT>,
}

impl<BottomT, BottomC, TopT, TopC> Caravel<DockTopTraveller<BottomT, TopT>>
for DockTopCaravel<BottomT, BottomC, TopT, TopC>
where BottomT: Traveller,
      BottomC: Caravel<BottomT>,
      TopT: Traveller,
      TopC: Caravel<TopT>
{
    fn embark(&self) -> DockTopTraveller<BottomT, TopT> {
        DockTopTraveller::new(self.top_units,
                              self.bottom_caravel.embark(),
                              self.top_caravel.embark())
    }
}

impl<BottomT, BottomC, TopT, TopC> DockTopCaravel<BottomT, BottomC, TopT, TopC>
where BottomT: Traveller,
      BottomC: Caravel<BottomT>,
      TopT: Traveller,
      TopC: Caravel<TopT>
{
    pub fn new(top_units: f32, bottom_caravel: BottomC, top_caravel: TopC) -> Self {
        DockTopCaravel {
            top_units: top_units,
            bottom_caravel: bottom_caravel,
            top_caravel: top_caravel,
            bottom_phantom: PhantomData,
            top_phantom: PhantomData,
        }
    }
}



