use journal::Journal;
use vrcounter::Patch;
use vrcounter::Sigil;
use rand;
use vrcounter::color::*;
use traveller::Traveller;

#[allow(dead_code)]
pub struct SpectrumTraveller {
    id: u64,
    color_index: usize,
}

impl SpectrumTraveller {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SpectrumTraveller { id: rand::random::<u64>(), color_index: 0 }
    }
}

impl Traveller for SpectrumTraveller {
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let cage = journal.screen_metrics().active_cage;
        let color = SPECTRUM[self.color_index / 2 % SPECTRUM.len()];
        let patch = Patch::from_cage(&cage, color, Sigil::Fill, self.id);
        journal.set_patch(patch.id, patch);
        self.color_index += 1;
    }
}