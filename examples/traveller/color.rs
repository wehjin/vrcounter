use journal::Journal;
use vrcounter::Sigil;
use traveller::{Traveller, patches_from_sigil, ids_from_sigil};

pub struct ColorTraveller {
    ids: Vec<u64>,
    color: [f32; 4],
    sigil: Sigil,
}

impl ColorTraveller {
    pub fn new(color: [f32; 4], sigil: Sigil) -> Self {
        let ids = ids_from_sigil(&sigil);
        ColorTraveller { ids: ids, color: color, sigil: sigil }
    }
}

impl Traveller for ColorTraveller {
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let cage = journal.screen_metrics().active_cage;
        let patches = patches_from_sigil(&self.sigil, &cage, self.color, &self.ids);
        for patch in patches {
            journal.set_patch(patch.id, patch);
        }
    }
}
