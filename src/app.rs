use color::{GREEN, RED, BLUE, CYAN, YELLOW, MAGENTA};
use viewer::{ActiveViewer};
use common::{IdSource};
use scream;
use scream::{ScreamPosition};
use howl;
use howl::Sigil;
use std::sync::mpsc::{channel};

pub struct AppModel {
    pub viewer: ActiveViewer,
}

impl AppModel {
    pub fn init(viewer: ActiveViewer) -> Self {
        let mut id_source = IdSource::new();
        let (message_tx, message_rx) = channel();
        let howls = [
            howl::start::<(), ()>(BLUE, Sigil::Fill, (-0.70, -0.50, -0.10, 0.10, 0.10, 0.10)),
            howl::start::<(), ()>(RED, Sigil::Fill, (-0.5, 0.5, -0.25, 0.25, 0.0, 0.0)),
            howl::start::<(), ()>(GREEN, Sigil::Fill, (0.25, 0.75, 0.0, 0.5, -0.01, -0.01)),
            howl::start::<(), ()>(CYAN, Sigil::Letter('J'), (-0.06, 0.00, -0.03, 0.03, 0.005, 0.005)),
            howl::start::<(), ()>(YELLOW, Sigil::Letter('y'), (0.00, 0.06, -0.03, 0.03, 0.005, 0.005)),
        ];
        for howl in howls.iter() {
            howl.present(viewer.clone(), message_tx.clone(), &mut id_source);
        }

        let position = ScreamPosition { left: -0.5, right: -0.4, top: -0.15, bottom: -0.25, near: 0.03 };
        let scream = scream::of_color(YELLOW)
            .join_right(0.1, scream::of_color(MAGENTA).join_right(0.1, scream::of_color(CYAN)));
        scream.present(&position, &mut id_source, viewer.clone());

        AppModel {
            viewer: viewer,
        }
    }
}
