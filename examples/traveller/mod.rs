pub mod color;
pub mod spectrum;
pub mod dock_top;
pub mod dock_left;

use journal::Journal;


pub trait Traveller {
    fn travel<J: Journal>(&mut self, journal: &mut J);
}

use cage;
use cage::Cage;
use vrcounter::Patch;
use vrcounter::PatchPosition;
use vrcounter::sigil::Sigil;

pub fn sigil_to_patch(sigil: &Sigil, cage: &Cage, color: [f32; 4], id: u64) -> Patch {
    let patch_width = if sigil.is_fill() {
        cage.frame.w
    } else {
        let width_per_height = sigil.width_per_height();
        cage.frame.h * width_per_height
    };
    let non_patch_width = cage.frame.w - patch_width;
    let patch_cage = cage.translate_sides(cage::Translation { right: -non_patch_width, ..Default::default() });
    Patch {
        id: id, glyph: sigil.ascii_point(), color: color, position: PatchPosition::from_cage(&patch_cage)
    }
}
