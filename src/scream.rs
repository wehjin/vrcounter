use std::collections::HashMap;
use std::char;
use color;

#[derive(Debug)]
struct PatchPosition {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32
}

#[derive(Debug)]
struct Patch {
    position: PatchPosition,
    color: [f32; 4],
    glyph: char,
    id: u32,
}

#[derive(Debug)]
struct Viewer {
    patch_map: HashMap<u32, Patch>,
}

impl Viewer {
    fn new() -> Self {
        Viewer { patch_map: HashMap::new() }
    }

    fn add_patch(&mut self, patch: Patch) {
        self.patch_map.insert(patch.id, patch);
    }
}

struct Closable;

#[derive(Debug)]
struct ScreamPosition {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32,
}

struct Scream<T: Fn(&ScreamPosition, &mut Viewer) -> Closable> {
    on_present: T
}

impl<T: Fn(&ScreamPosition, &mut Viewer) -> Closable> Scream<T> {
    fn create(on_present: T) -> Scream<T> {
        Scream {
            on_present: on_present,
        }
    }

    fn present(&self, position: &ScreamPosition, viewer: &mut Viewer) -> Closable {
        let on_present = &(self.on_present);
        on_present(position, viewer)
    }
}

fn on_present_color(position: &ScreamPosition, viewer: &mut Viewer) -> Closable {
    let patch_position = PatchPosition {
        left: position.left, right: position.right, top: position.top, bottom: position.bottom,
        near: position.near
    };
    let patch = Patch { position: patch_position, color: color::MAGENTA, glyph: 'Z', id: 27u32 };
    viewer.add_patch(patch);
    Closable {}
}

pub fn main() {
    let scream = Scream::create(on_present_color);
    let mut viewer = Viewer::new();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: 0.5, bottom: 0.4, near: 0.05 };
    scream.present(&position, &mut viewer);
    println!("Viewer: {:?}", viewer);
}

