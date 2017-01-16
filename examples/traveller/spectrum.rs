use journal::Journal;
use vrcounter::Sigil;
use vrcounter::color::*;
use traveller::{Traveller, patches_from_sigil, ids_from_sigil};

#[allow(dead_code)]
pub struct SpectrumTraveller {
    ids: Vec<u64>,
    color_index: usize,
    sigil: Sigil,
}

impl SpectrumTraveller {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let sigil = Sigil::of_fill();
        let ids = ids_from_sigil(&sigil);
        SpectrumTraveller { ids: ids, color_index: 0, sigil: sigil }
    }
}

impl Traveller for SpectrumTraveller {
    fn travel<J: Journal>(&mut self, journal: &mut J) {
        let cage = journal.screen_metrics().active_cage;
        let color = SPECTRUM[self.color_index / 2 % SPECTRUM.len()];
        let patch = patches_from_sigil(&self.sigil, &cage, color, &self.ids)[0];
        journal.set_patch(patch.id, patch);
        self.color_index += 1;
    }
}