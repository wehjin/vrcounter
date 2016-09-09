use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;
use patch::*;

enum Message {
    AddPatch(Patch),
    RemovePatch(u64),
    SendPatchReport(Sender<HashMap<u64, Patch>>),
    Stop,
}

#[derive(Clone)]
pub struct ActiveViewer {
    command_tx: Sender<Message>,
}

impl ActiveViewer {
    pub fn start() -> Self {
        let (tx, rx) = channel();
        let mut patches = HashMap::new();
        thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                match message {
                    Message::AddPatch(patch) => {
                        patches.insert(patch.id, patch);
                    },
                    Message::RemovePatch(id) => {
                        patches.remove(&id);
                    },
                    Message::SendPatchReport(report_tx) => {
                        report_tx.send(patches.clone()).unwrap();
                    },
                    Message::Stop => {
                        break;
                    }
                }
            }
        });
        ActiveViewer { command_tx: tx }
    }
    pub fn add_patch(&self, patch: Patch) {
        self.command_tx.send(Message::AddPatch(patch)).unwrap();
    }
    pub fn remove_patch(&self, id: u64) {
        self.command_tx.send(Message::RemovePatch(id)).unwrap();
    }
    pub fn get_patch_report(&self) -> HashMap<u64, Patch> {
        let (report_tx, report_rx) = channel();
        self.command_tx.send(Message::SendPatchReport(report_tx)).unwrap();
        if let Ok(report) = report_rx.recv() { report } else { HashMap::new() }
    }
    pub fn stop(&self) {
        self.command_tx.send(Message::Stop).unwrap_or(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use patch::{Sigil, Patch};
    use color::MAGENTA;

    #[test]
    fn add_patch() {
        let viewer = ActiveViewer::start();
        let patch = Patch::new(1, -1.0, 1.0, -1.0, 1.0, 0.0, MAGENTA, Sigil::Fill);
        viewer.add_patch(patch);
        let report = viewer.get_patch_report();
        viewer.stop();
        assert!(report.contains_key(&1));
    }
}