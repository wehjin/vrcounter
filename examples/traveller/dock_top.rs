use journal::Journal;
use traveller::Traveller;
use cage::{Cage, Translation};
use journal::CageJournal;

pub struct DockTopTraveller<BottomT: Traveller, TopT: Traveller> {
    top_units: f32,
    bottom_traveller: BottomT,
    top_traveller: TopT,
}

impl<BottomT, TopT> DockTopTraveller<BottomT, TopT>
where BottomT: Traveller,
      TopT: Traveller
{
    pub fn new(top_units: f32, bottom_traveller: BottomT, top_traveller: TopT) -> Self {
        DockTopTraveller {
            top_units: top_units,
            bottom_traveller: bottom_traveller,
            top_traveller: top_traveller,
        }
    }
}

impl<BottomT, TopT> Traveller for DockTopTraveller<BottomT, TopT>
where BottomT: Traveller,
      TopT: Traveller
{
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let screen_metrics = journal.screen_metrics();
        let cage = screen_metrics.active_cage;
        let top_height = screen_metrics.preferred_reading_height * self.top_units;
        let (top_cage, bottom_cage) = divide_cage(cage, top_height);
        // TODO move to function
        {
            let mut bottom_journal = CageJournal::new(bottom_cage, journal);
            self.bottom_traveller.travel(&mut bottom_journal);
        }
        {
            let mut top_journal = CageJournal::new(top_cage, journal);
            self.top_traveller.travel(&mut top_journal);
        }
    }
}

fn divide_cage(cage: Cage, top_height: f32) -> (Cage, Cage) {
    let bottom_height = cage.frame.h - top_height;
    let bottom_cage = cage.translate_sides(Translation { top: -top_height, ..Default::default() });
    let top_cage = cage.translate_sides(Translation { bottom: bottom_height, ..Default::default() });
    (top_cage, bottom_cage)
}
