use std::collections::HashMap;
use std::char;
use color::*;

#[derive(Debug)]
pub struct PatchPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32
}

#[derive(Debug)]
pub struct Patch {
    pub position: PatchPosition,
    pub color: [f32; 4],
    pub glyph: char,
    pub id: u64,
}

#[derive(Debug)]
pub struct Viewer {
    pub patch_map: HashMap<u64, Patch>,
    global_id: u64,
}

impl Viewer {
    pub fn new() -> Self {
        Viewer { patch_map: HashMap::new(), global_id: 1 }
    }
    pub fn next_id(&mut self) -> u64 {
        let id = self.global_id;
        self.global_id += 1;
        id
    }
    pub fn add_patch(&mut self, patch: Patch) {
        self.patch_map.insert(patch.id, patch);
    }
    pub fn remove_patch(&mut self, id: u64) {
        self.patch_map.remove(&id);
    }
}

pub struct Presenting {
    on_stop: Box<Fn()>
}

impl Presenting {
    fn stop(&self) {
        (*self.on_stop)();
    }
    fn create(on_stop: Box<Fn()>) -> Self {
        Presenting { on_stop: on_stop }
    }
    fn empty() -> Self {
        Presenting { on_stop: Box::new(move || {}) }
    }
    fn double(first: Presenting, second: Presenting) -> Self {
        Presenting {
            on_stop: Box::new(move || {
                first.stop();
                second.stop();
            })
        }
    }
}

#[derive(Debug)]
pub struct ScreamPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
}

pub struct Scream {
    on_present: Box<Fn(&ScreamPosition, &mut Viewer) -> Presenting>
}

impl Scream {
    pub fn create(on_present: Box<Fn(&ScreamPosition, &mut Viewer) -> Presenting>) -> Self {
        Scream { on_present: on_present }
    }
    pub fn present(&self, position: &ScreamPosition, viewer: &mut Viewer) -> Presenting {
        let on_present = &(self.on_present);
        on_present(position, viewer)
    }

    pub fn join_right(self, width: f32, right_scream: Scream) -> Scream {
        let on_present = move |position: &ScreamPosition, viewer: &mut Viewer| -> Presenting {
            let left_presenting = self.present(position, viewer);
            let &ScreamPosition { right: right, top: top, bottom: bottom, near: near, .. } = position;
            let right_position = ScreamPosition { left: right, right: right + width, top: top, bottom: bottom, near: near };
            let right_presenting = right_scream.present(&right_position, viewer);
            Presenting::double(left_presenting, right_presenting)
        };
        Scream::create(Box::new(on_present))
    }
}

pub fn of_color(color: [f32; 4]) -> Scream {
    Scream::create(Box::new(move |position: &ScreamPosition, viewer: &mut Viewer| -> Presenting {
        present_color(position, viewer, color)
    }))
}

fn present_color(position: &ScreamPosition, viewer: &mut Viewer, color: [f32; 4]) -> Presenting {
    let patch_position = PatchPosition {
        left: position.left, right: position.right, top: position.top, bottom: position.bottom,
        near: position.near
    };
    let id = viewer.next_id();
    let patch = Patch { position: patch_position, color: color, glyph: 'Z', id: id };
    viewer.add_patch(patch);
    Presenting::create(Box::new(move || {
        // TODO Remove patch
        //viewer.remove_patch(id);
    }))
}

#[test]
pub fn main() {
    let scream = of_color(color::MAGENTA);
    let mut viewer = Viewer::new();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: 0.5, bottom: 0.4, near: 0.05 };
    scream.present(&position, &mut viewer);
    println!("Viewer: {:?}", viewer);
    assert_eq!(viewer.patch_map.len(), 1)
}

