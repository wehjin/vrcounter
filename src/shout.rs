use std::sync::mpsc::{Sender};
use viewer::{Viewer, IdSource, Patch, PatchPosition};
use color;

pub enum Message<T, E> {
    Position {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        far: f32,
        near: f32,
    },
    Ok(T),
    Err(E),
}

pub struct Shout<T, E> {
    on_present: Box<Fn(Viewer, Sender<Message<T, E>>, &mut IdSource) -> Shouting>,
}

impl<T, E> Shout<T, E> {
    pub fn create(on_present: Box<Fn(Viewer, Sender<Message<T, E>>, &mut IdSource) -> Shouting>) -> Self {
        Shout { on_present: on_present }
    }
    pub fn present(&self, viewer: Viewer, sender: Sender<Message<T, E>>, id_source: &mut IdSource) -> Shouting {
        let on_present = &(self.on_present);
        on_present(viewer, sender, id_source)
    }
}

pub struct Shouting {
    is_silenced: bool,
    on_silence: Box<Fn()>,
}

impl Shouting {
    pub fn new(on_silence: Box<Fn()>) -> Self {
        Shouting { is_silenced: false, on_silence: on_silence }
    }
    pub fn is_silenced(&self) -> bool {
        self.is_silenced
    }
    pub fn silence(&mut self) {
        if self.is_silenced {} else {
            self.is_silenced = true;
            (&self.on_silence)();
        }
    }
}

pub fn create<T, E>(color: [f32; 4], ) -> Shout<T, E> {
    let (left, right, bottom, top, far, near) = (-0.70, -0.50, -0.10, 0.10, 0.10, 0.10);
    Shout::create(Box::new(move |viewer: Viewer, sender: Sender<Message<T, E>>, id_source: &mut IdSource| -> Shouting {
        let position = PatchPosition { left: left, right: right, bottom: bottom, top: top, near: near };
        let id = id_source.next_id();
        let patch = Patch::of_color(&position, &color, id);
        viewer.add_patch(patch);
        sender.send(Message::Position {
            left: left, right: right, bottom: bottom, top: top, far: far, near: near
        }).unwrap();
        Shouting::new(Box::new(move || { viewer.remove_patch(id); }))
    }))
}

#[cfg(test)]
mod tests {
    use super::{Shout, Shouting, Message};
    use viewer::{Viewer, IdSource, Patch, PatchPosition};
    use std::sync::mpsc::{Sender, channel};
    use color;

    #[test]
    fn it_works() {
        let viewer = Viewer::start();
        let (sender, receiver) = channel();
        let mut id_source = IdSource::new();
        let mut shouting = shout.present(viewer.clone(), sender.clone(), &mut id_source);
        let received_position = receiver.recv().unwrap();
        let received_position_match = match received_position {
            Message::Position {
                left: 0.0,
                right: 10.0,
                bottom: 0.0,
                top: 20.0,
                far: 0.01,
                near: 0.01,
            } => true,
            _ => false,
        };
        assert!(received_position_match);

        let received_result = receiver.recv().unwrap();
        let received_result_match = match received_result {
            Message::Ok(33u32) => true,
            _ => false,
        };
        assert!(received_result_match);
        shouting.silence();
        viewer.stop();
    }
}