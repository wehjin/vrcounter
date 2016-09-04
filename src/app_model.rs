use shape::{Shape, ShapeList, ShapeMask};
use color::{GREEN, RED, BLUE, CYAN, YELLOW, MAGENTA};
use viewer::{ActiveViewer};
use common::{IdSource};
use scream;
use scream::{ScreamPosition};
use howl;
use howl::Message as HowlMessage;
use std::sync::mpsc::{channel};

pub struct AppModel {
    pub shape_list: ShapeList,
}

impl AppModel {
    pub fn init() -> Self {
        let mut shape_list = ShapeList::new();
        shape_list.push(Shape::new(-0.5, 0.5, 0.25, -0.25, 0.0, RED, 0, ShapeMask::None));
        shape_list.push(Shape::new(0.25, 0.75, 0.5, 0.0, -0.01, GREEN, 1, ShapeMask::None));
        shape_list.push(Shape::new(-0.06, 0.00, 0.03, -0.03, 0.005, CYAN, 2, ShapeMask::Letter('J')));
        shape_list.push(Shape::new(0.00, 0.06, 0.03, -0.03, 0.005, YELLOW, 2, ShapeMask::Letter('y')));
        let more_shapes = get_shapes();
        for shape in more_shapes {
            shape_list.push(shape);
        }
        AppModel {
            shape_list: shape_list
        }
    }
}

fn get_shapes() -> Vec<Shape> {
    let mut shapes = Vec::new();
    let viewer = ActiveViewer::start();
    let mut id_source = IdSource::new();
    let position = ScreamPosition { left: -0.5, right: -0.4, top: -0.15, bottom: -0.25, near: 0.03 };
    let scream = scream::of_color(YELLOW)
        .join_right(0.1, scream::of_color(MAGENTA)
            .join_right(0.1, scream::of_color(CYAN))
        );
    scream.present(&position, &mut id_source, viewer.clone());

    let howl = howl::create_color::<(), ()>(BLUE);
    let (message_tx, message_rx) = channel();
    howl.present(viewer.clone(), message_tx, &mut id_source);
    let howl_message = message_rx.recv().unwrap();
    match howl_message {
        HowlMessage::Position { .. } => {
            println!("Howl position")
        },
        _ => println!("Other message")
    }

    let patch_map = viewer.get_patch_report();
    for (_, patch) in patch_map {
        let mask = if patch.glyph == '\u{0}' { ShapeMask::None } else { ShapeMask::Letter(patch.glyph) };
        let shape = Shape::new(patch.position.left, patch.position.right,
                               patch.position.top, patch.position.bottom,
                               patch.position.near, patch.color,
                               patch.id, mask);
        shapes.push(shape);
    }
    viewer.stop();
    shapes
}
