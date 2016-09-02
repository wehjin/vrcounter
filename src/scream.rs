use viewer::{Viewer, Patch, PatchPosition, IdSource};
use color::MAGENTA;

#[derive(Debug)]
pub struct ScreamPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
}

pub struct Scream {
    on_present: Box<Fn(&ScreamPosition, &mut IdSource, Viewer) -> Presenting>
}

impl Scream {
    pub fn create(on_present: Box<Fn(&ScreamPosition, &mut IdSource, Viewer) -> Presenting>) -> Self {
        Scream { on_present: on_present }
    }
    pub fn present(&self, position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer) -> Presenting {
        let on_present = &(self.on_present);
        on_present(position, id_source, viewer)
    }

    pub fn join_right(self, width: f32, right_scream: Scream) -> Scream {
        let on_present = move |position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer| -> Presenting {
            let left_presenting = self.present(position, id_source, viewer.clone());
            let &ScreamPosition { right, top, bottom, near, .. } = position;
            let right_position = ScreamPosition { left: right, right: right + width, top: top, bottom: bottom, near: near };
            let right_presenting = right_scream.present(&right_position, id_source, viewer.clone());
            Presenting::double(left_presenting, right_presenting)
        };
        Scream::create(Box::new(on_present))
    }
}

pub struct Presenting {
    on_stop: Box<Fn()>
}

impl Presenting {
    pub fn stop(&self) {
        (*self.on_stop)();
    }
    pub fn create(on_stop: Box<Fn()>) -> Self {
        Presenting { on_stop: on_stop }
    }
    pub fn empty() -> Self {
        Presenting { on_stop: Box::new(move || {}) }
    }
    pub fn double(first: Presenting, second: Presenting) -> Self {
        Presenting {
            on_stop: Box::new(move || {
                first.stop();
                second.stop();
            })
        }
    }
}

pub fn of_color(color: [f32; 4]) -> Scream {
    Scream::create(Box::new(move |position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer| -> Presenting {
        present_color(position, id_source, viewer, color)
    }))
}

fn present_color(position: &ScreamPosition, id_source: &mut IdSource, viewer: Viewer, color: [f32; 4]) -> Presenting {
    let patch_position = PatchPosition {
        left: position.left, right: position.right, top: position.top, bottom: position.bottom,
        near: position.near
    };
    let id = id_source.next_id();
    let patch = Patch { position: patch_position, color: color, glyph: 'Z', id: id };
    viewer.add_patch(patch);
    Presenting::create(Box::new(move || {
        viewer.remove_patch(id);
    }))
}

#[test]
pub fn main() {
    let scream = of_color(MAGENTA);
    let viewer = Viewer::start();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: 0.5, bottom: 0.4, near: 0.05 };
    let mut id_source = IdSource::new();
    scream.present(&position, &mut id_source, viewer.clone());
    let report = viewer.get_report();
    let report_length = report.len();
    assert_eq!(report_length, 1)
}

