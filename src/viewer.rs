use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::collections::HashMap;
use patch::Patch;
use mist::Mist;
use hand::Hand;

enum Message {
    AddPatch(Patch),
    RemovePatch(u64),
    SendPatches(Sender<HashMap<u64, Patch>>),
    AddMist(Mist),
    SendMists(Sender<HashMap<u64, Mist>>),
    SendHand(Sender<Hand>),
    Clear,
    Stop,
}

#[derive(Clone)]
pub struct ActiveViewer {
    command_tx: Sender<Message>,
}

impl ActiveViewer {
    pub fn start() -> Self {
        let (tx, rx) = channel();
        thread::spawn(move || {
            let mut patches = HashMap::new();
            let mut mists = HashMap::new();
            let hand: Hand = Default::default();
            while let Ok(message) = rx.recv() {
                match message {
                    Message::Clear => { mists.clear(); }
                    Message::AddPatch(patch) => { patches.insert(patch.id, patch); },
                    Message::RemovePatch(id) => { patches.remove(&id); },
                    Message::SendPatches(tx) => { tx.send(patches.clone()).unwrap(); },
                    Message::AddMist(mist) => { mists.insert(mist.id(), mist); },
                    Message::SendMists(tx) => { tx.send(mists.clone()).unwrap(); },
                    Message::SendHand(tx) => { tx.send(hand.clone()).unwrap(); },
                    Message::Stop => { break; }
                }
            }
        });
        ActiveViewer { command_tx: tx }
    }

    pub fn get_patches(&self) -> HashMap<u64, Patch> {
        let (tx, rx) = channel();
        self.command_tx.send(Message::SendPatches(tx)).unwrap();
        if let Ok(patches) = rx.recv() { patches } else { Default::default() }
    }
    pub fn get_mists(&self) -> HashMap<u64, Mist> {
        let (tx, rx) = channel();
        self.command_tx.send(Message::SendMists(tx)).unwrap();
        if let Ok(mists) = rx.recv() { mists } else { Default::default() }
    }
    pub fn get_hand(&self) -> Hand {
        let (tx, rx) = channel();
        self.command_tx.send(Message::SendHand(tx)).unwrap();
        if let Ok(hand) = rx.recv() { hand } else { Default::default() }
    }
    pub fn add_patch(&self, patch: Patch) { self.command_tx.send(Message::AddPatch(patch)).unwrap(); }
    pub fn remove_patch(&self, id: u64) { self.command_tx.send(Message::RemovePatch(id)).unwrap(); }
    pub fn add_mist(&self, mist: Mist) { self.command_tx.send(Message::AddMist(mist)).unwrap(); }
    pub fn clear(&self) { self.command_tx.send(Message::Clear).unwrap(); }
    pub fn stop(&self) { self.command_tx.send(Message::Stop).unwrap_or(()); }
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
        let report = viewer.get_patches();
        viewer.stop();
        assert!(report.contains_key(&1));
    }


    #[test]
    fn add_mist() {
        use mist::{Mist};
        use cage::{Cage};

        let viewer = ActiveViewer::start();
        let (mist, mist_rx) = Mist::new(2, Cage::from((0.0, 0.1, 0.0, 0.1, 0.0, 0.1)));
        viewer.add_mist(mist);
        let report = viewer.get_mists();
        viewer.stop();
        assert!(report.contains_key(&2));
    }
}