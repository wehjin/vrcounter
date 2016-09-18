extern crate cage;

use cage::*;
use std::boxed::Box;
use vision::Vision;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use viewer::Viewer;
use std::thread;
use os;
use gl_user;
use gl_user::Emission as UserEmission;
use vr_user;


pub trait Sun {
    fn fade(&mut self);
}

pub trait Scene<T> {
    fn revise(&mut self, vision: Vision<T>);
}

pub trait FrameStar<T> {
    fn summon(&self, visioner: &mut Scene<T>, offset: &Offset) -> Box<FrameSun>;
}

pub trait FrameSun: Sun {
    fn update_offset(&mut self, offset: &Offset);
}

pub struct UserScene<T> {
    viewer: Viewer,
    vision: Option<Vision<T>>,
}

impl<T> Scene<T> for UserScene<T> {
    fn revise(&mut self, vision: Vision<T>) {
        for (_, patch) in &vision.patches {
            self.viewer.add_patch(patch.clone());
        }
        for (_, mist) in &vision.mists {
            self.viewer.add_mist(mist.clone());
        }
        self.vision = Some(vision);
    }
}

impl<T> UserScene<T> {
    pub fn start<F>(emitter: F) -> Self where F: Fn(UserEmission) + Send + 'static {
        let viewer = Viewer::start();
        let thread_viewer = viewer.clone();
        thread::spawn(move || {
            if os::is_windows() {
                vr_user::run(thread_viewer.clone(), Arc::new(emitter));
            } else {
                gl_user::run(thread_viewer.clone(), Arc::new(emitter));
            }
            thread_viewer.stop();
        });
        UserScene { viewer: viewer, vision: None }
    }
    fn stop(&mut self) {}
}

