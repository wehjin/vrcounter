use caravel::Caravel;
use traveller::Traveller;
use traveller::top_dock::TopDockTraveller;
use std::marker::PhantomData;

pub struct TopDockCaravel<BottomT: Traveller, BottomC: Caravel<BottomT>, TopT: Traveller, TopC: Caravel<TopT>>
{
    top_units: f32,
    bottom_caravel: BottomC,
    top_caravel: TopC,
    top_phantom: PhantomData<BottomT>,
    bottom_phantom: PhantomData<TopT>,
}

impl<BottomT, BottomC, TopT, TopC> Caravel<TopDockTraveller<BottomT, TopT>>
for TopDockCaravel<BottomT, BottomC, TopT, TopC>
where BottomT: Traveller,
      BottomC: Caravel<BottomT>,
      TopT: Traveller,
      TopC: Caravel<TopT>
{
    fn embark(&self) -> TopDockTraveller<BottomT, TopT> {
        TopDockTraveller::new(self.top_units,
                              self.bottom_caravel.embark(),
                              self.top_caravel.embark())
    }
}

impl<BottomT, BottomC, TopT, TopC> TopDockCaravel<BottomT, BottomC, TopT, TopC>
where BottomT: Traveller,
      BottomC: Caravel<BottomT>,
      TopT: Traveller,
      TopC: Caravel<TopT>
{
    pub fn new(top_units: f32, bottom_caravel: BottomC, top_caravel: TopC) -> Self {
        TopDockCaravel {
            top_units: top_units,
            bottom_caravel: bottom_caravel,
            top_caravel: top_caravel,
            bottom_phantom: PhantomData,
            top_phantom: PhantomData,
        }
    }
}



