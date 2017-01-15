use journal::Journal;
use traveller::Traveller;
use cage::{Cage, Translation};
use journal::CageJournal;

pub struct DockLeftTraveller<LeftT: Traveller, RightT: Traveller> {
    left_units: f32,
    left_traveller: LeftT,
    right_traveller: RightT,
}

impl<LeftT, RightT> DockLeftTraveller<LeftT, RightT>
where LeftT: Traveller,
      RightT: Traveller
{
    pub fn new(left_units: f32, left_traveller: LeftT, right_traveller: RightT) -> Self {
        DockLeftTraveller {
            left_units: left_units,
            left_traveller: left_traveller,
            right_traveller: right_traveller,
        }
    }
}

impl<LeftT, RightT> Traveller for DockLeftTraveller<LeftT, RightT>
where LeftT: Traveller,
      RightT: Traveller
{
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let screen_metrics = journal.screen_metrics();
        let cage = screen_metrics.active_cage;
        let left_width = screen_metrics.preferred_reading_height * self.left_units;
        let (left_cage, right_cage) = divide_cage(cage, left_width);
        // TODO move to function
        {
            let mut journal = CageJournal::new(left_cage, journal);
            self.left_traveller.travel(&mut journal);
        }
        {
            let mut journal = CageJournal::new(right_cage, journal);
            self.right_traveller.travel(&mut journal);
        }
    }
}

fn divide_cage(cage: Cage, left_width: f32) -> (Cage, Cage) {
    let right_width = cage.frame.w - left_width;
    let left_cage = cage.translate_sides(Translation { right: -right_width, ..Default::default() });
    let right_cage = cage.translate_sides(Translation { left: left_width, ..Default::default() });
    (left_cage, right_cage)
}
