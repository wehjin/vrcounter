use viewer::{Viewer};
use patch::{Patch, PatchPosition};
use common::{IdSource};

#[derive(Debug)]
pub struct ScreamPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
}

pub struct OScream {
    on_present: Box<Fn(&ScreamPosition, &mut IdSource, Viewer) -> OScreaming>
}

impl OScream {
    pub fn create(on_present: Box<Fn(&ScreamPosition, &mut IdSource, Viewer) -> OScreaming>) -> Self {
        OScream { on_present: on_present }
    }
    pub fn present(&self, position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer) -> OScreaming {
        let on_present = &(self.on_present);
        on_present(position, id_source, viewer)
    }

    pub fn join_right(self, width: f32, right_scream: OScream) -> OScream {
        let on_present = move |position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer| -> OScreaming {
            let left_presenting = self.present(position, id_source, viewer.clone());
            let &ScreamPosition { right, top, bottom, near, .. } = position;
            let right_position = ScreamPosition { left: right, right: right + width, top: top, bottom: bottom, near: near };
            let right_presenting = right_scream.present(&right_position, id_source, viewer.clone());
            OScreaming::double(left_presenting, right_presenting)
        };
        OScream::create(Box::new(on_present))
    }
}

pub struct OScreaming {
    on_stop: Box<Fn()>
}

impl OScreaming {
    pub fn silence(&self) {
        (*self.on_stop)();
    }
    pub fn create(on_stop: Box<Fn()>) -> Self {
        OScreaming { on_stop: on_stop }
    }
    pub fn double(first: OScreaming, second: OScreaming) -> Self {
        OScreaming {
            on_stop: Box::new(move || {
                first.silence();
                second.silence();
            })
        }
    }
}

pub fn of_color(color: [f32; 4]) -> OScream {
    OScream::create(Box::new(move |position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer| -> OScreaming {
        present_color(position, id_source, viewer, color)
    }))
}

fn present_color(position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer, color: [f32; 4]) -> OScreaming {
    let patch_position = PatchPosition {
        left: position.left, right: position.right, top: position.top, bottom: position.bottom,
        near: position.near
    };
    let id = id_source.id();
    let patch = Patch { position: patch_position, color: color, glyph: 'Z', id: id };
    viewer.add_patch(patch);
    OScreaming::create(Box::new(move || {
        viewer.remove_patch(id);
    }))
}

#[test]
pub fn main() {
    use color;
    let scream = of_color(color::MAGENTA);
    let viewer = Viewer::start();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: 0.5, bottom: 0.4, near: 0.05 };
    let mut id_source = IdSource::new();
    scream.present(&position, &mut id_source, viewer.clone());
    let report = viewer.get_patches();
    let report_length = report.len();
    assert_eq!(report_length, 1)
}

