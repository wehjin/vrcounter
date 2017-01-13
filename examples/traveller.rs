use journal::Journal;
use vrcounter::Patch;
use vrcounter::Sigil;
use vrcounter::color::*;
use rand;

pub trait Traveller {
    fn travel<J: Journal>(&mut self, &mut J);
}

pub struct PatchTraveller {
    id: u64,
    color: [f32; 4],
}

impl PatchTraveller {
    pub fn new() -> Self {
        let color = SPECTRUM[1 % SPECTRUM.len()];
        PatchTraveller { id: rand::random::<u64>(), color: color }
    }
}

impl Traveller for PatchTraveller {
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let cage = journal.screen_metrics().active_cage;
        let patch = Patch::from_cage(&cage, self.color, Sigil::Fill, self.id);
        journal.set_patch(patch.id, patch);
    }
}
