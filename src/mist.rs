use cage::{Cage};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::cell::{Cell};

pub enum Message {
    In(u64, f32, f32, f32),
    Out,
}

#[derive(Clone, Debug)]
pub struct Mist {
    id: u64,
    cage: Cage,
    on_message: Sender<Message>,
    lifted: Cell<bool>
}

impl Mist {
    pub fn new(id: u64, cage: Cage) -> (Self, Receiver<Message>) {
        let (sender, receiver) = channel::<Message>();
        let mist = Mist { id: id, cage: cage, on_message: sender, lifted: Cell::new(false) };
        (mist, receiver)
    }
    pub fn touch(&self, x: f32, y: f32, z: f32) -> bool {
        if self.lifted.get() || !self.cage.contains(x, y, z) {
            false
        } else {
            let message = Message::In(self.id, x, y, z);
            if self.on_message.send(message).is_err() {
                self.lifted.set(false);
                false
            } else {
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cage::{Cage};
    use std::sync::mpsc::{channel, Receiver};
    use super::*;

    #[test]
    fn mist_touch() {
        let cage: Cage = Default::default();
        let (mist, receiver): (Mist, Receiver<Message>) = Mist::new(1, cage);
        let touched = mist.touch(0.0, 0.0, 0.0);
        assert!(touched);
        let message = receiver.recv().unwrap();
        assert!(match message {
            Message::In(id, x, y, z) => id == 1 && x == 0.0 && y == 0.0 && z == 0.0,
            _ => false

        });
    }
}