use std::collections::HashMap;
use std::char;
use color;

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
}

impl Viewer {
    pub fn new() -> Self {
        Viewer { patch_map: HashMap::new() }
    }

    pub fn add_patch(&mut self, patch: Patch) {
        self.patch_map.insert(patch.id, patch);
    }
}

pub struct Closable;

#[derive(Debug)]
pub struct ScreamPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
}

pub struct Scream<T: Fn(&ScreamPosition, &mut Viewer) -> Closable> {
    on_present: T
}

impl<T: Fn(&ScreamPosition, &mut Viewer) -> Closable> Scream<T> {
    pub fn create(on_present: T) -> Scream<T> {
        Scream {
            on_present: on_present,
        }
    }

    pub fn present(&self, position: &ScreamPosition, viewer: &mut Viewer) -> Closable {
        let on_present = &(self.on_present);
        on_present(position, viewer)
    }
}

pub fn on_present_color(position: &ScreamPosition, viewer: &mut Viewer) -> Closable {
    let patch_position = PatchPosition {
        left: position.left, right: position.right, top: position.top, bottom: position.bottom,
        near: position.near
    };
    let patch = Patch { position: patch_position, color: color::MAGENTA, glyph: 'Z', id: 27u64 };
    viewer.add_patch(patch);
    Closable {}
}

#[test]
fn it_works() {
    let scream = Scream::create(on_present_color);
    let mut viewer = Viewer::new();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: 0.5, bottom: 0.4, near: 0.05 };
    scream.present(&position, &mut viewer);
    println!("Viewer: {:?}", viewer);
    assert_eq!(viewer.patch_map.len(), 1)
}

pub fn main() {
    let scream = Scream::create(on_present_color);
    let mut viewer = Viewer::new();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: 0.5, bottom: 0.4, near: 0.05 };
    scream.present(&position, &mut viewer);
    println!("Viewer: {:?}", viewer);
}

