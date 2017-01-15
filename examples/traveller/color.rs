use journal::Journal;
use vrcounter::Sigil;
use rand;
use traveller::{Traveller, sigil_to_patch};

pub struct ColorTraveller {
    id: u64,
    color: [f32; 4],
    sigil: Sigil,
}

impl ColorTraveller {
    pub fn new(color: [f32; 4], sigil: Sigil) -> Self {
        ColorTraveller {
            id: rand::random::<u64>(),
            color: color,
            sigil: sigil,
        }
    }
}

impl Traveller for ColorTraveller {
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let cage = journal.screen_metrics().active_cage;
        let patch = sigil_to_patch(&self.sigil, &cage, self.color, self.id);
        journal.set_patch(patch.id, patch);
    }
}
