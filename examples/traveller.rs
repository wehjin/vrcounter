use journal::Journal;
use vrcounter::Patch;
use vrcounter::Sigil;
use rand;

pub trait Traveller {
    fn travel<J: Journal>(&mut self, &mut J);
}

pub struct ColorTraveller {
    id: u64,
    color: [f32; 4],
}

impl ColorTraveller {
    pub fn new(color: [f32; 4]) -> Self {
        ColorTraveller { id: rand::random::<u64>(), color: color }
    }
}

impl Traveller for ColorTraveller {
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let cage = journal.screen_metrics().active_cage;
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, self.id);
        journal.set_patch(patch.id, patch);
    }
}
