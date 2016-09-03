use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;
use patch::*;

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

enum Message {
    AddPatch(Patch),
    RemovePatch(u64),
    SendReport(Sender<HashMap<u64, Patch>>),
    Stop,
}

#[derive(Clone)]
pub struct Viewer {
    sender: Sender<Message>,
}

impl Viewer {
    pub fn start() -> Self {
        let (sender, receiver) = channel();
        let mut patches = HashMap::new();
        thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                match message {
                    Message::AddPatch(patch) => {
                        patches.insert(patch.id, patch);
                    },
                    Message::RemovePatch(id) => {
                        patches.remove(&id);
                    },
                    Message::SendReport(report_sender) => {
                        report_sender.send(patches.clone()).unwrap();
                    },
                    Message::Stop => {
                        break;
                    }
                }
            }
        });
        Viewer { sender: sender }
    }
    pub fn add_patch(&self, patch: Patch) {
        self.sender.send(Message::AddPatch(patch)).unwrap();
    }
    pub fn remove_patch(&self, id: u64) {
        self.sender.send(Message::RemovePatch(id)).unwrap();
    }
    pub fn get_report(&self) -> HashMap<u64, Patch> {
        let (report_sender, report_receiver) = channel();
        self.sender.send(Message::SendReport(report_sender)).unwrap();
        if let Ok(patches) = report_receiver.recv() {
            patches
        } else {
            HashMap::new()
        }
    }
    pub fn stop(&self) {
        self.sender.send(Message::Stop).unwrap();
    }
}
