use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;

#[derive(Debug)]
pub struct IdSource {
    global_id: u64,
}

impl IdSource {
    pub fn new() -> Self {
        IdSource { global_id: 1u64 }
    }
    pub fn next_id(&mut self) -> u64 {
        let id = self.global_id;
        self.global_id = self.global_id + 1;
        id
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PatchPosition {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32
}

#[derive(Debug, Copy, Clone)]
pub struct Patch {
    pub position: PatchPosition,
    pub color: [f32; 4],
    pub glyph: char,
    pub id: u64,
}

impl Patch {
    pub fn of_color(position: &PatchPosition, color: &[f32; 4], id: u64) -> Self {
        Patch {
            position: position.clone(),
            color: color.clone(),
            glyph: 'X',
            id: id
        }
    }
}

enum ViewerMessage {
    AddPatch(Patch),
    RemovePatch(u64),
    SendReport(Sender<HashMap<u64, Patch>>),
    Stop,
}

#[derive(Clone)]
pub struct Viewer {
    sender: Sender<ViewerMessage>,
}

impl Viewer {
    pub fn start() -> Self {
        let (sender, receiver) = channel();
        let mut patches = HashMap::new();
        thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                match message {
                    ViewerMessage::AddPatch(patch) => {
                        patches.insert(patch.id, patch);
                    },
                    ViewerMessage::RemovePatch(id) => {
                        patches.remove(&id);
                    },
                    ViewerMessage::SendReport(report_sender) => {
                        report_sender.send(patches.clone()).unwrap();
                    },
                    ViewerMessage::Stop => {
                        break;
                    }
                }
            }
        });
        Viewer { sender: sender }
    }
    pub fn add_patch(&self, patch: Patch) {
        self.sender.send(ViewerMessage::AddPatch(patch)).unwrap();
    }
    pub fn remove_patch(&self, id: u64) {
        self.sender.send(ViewerMessage::RemovePatch(id)).unwrap();
    }
    pub fn get_report(&self) -> HashMap<u64, Patch> {
        let (report_sender, report_receiver) = channel();
        self.sender.send(ViewerMessage::SendReport(report_sender)).unwrap();
        if let Ok(patches) = report_receiver.recv() {
            patches
        } else {
            HashMap::new()
        }
    }
    pub fn stop(&self) {
        self.sender.send(ViewerMessage::Stop).unwrap();
    }
}
