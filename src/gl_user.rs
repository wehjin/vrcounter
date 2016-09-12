extern crate glium;

use glium::{DisplayBuild, Display, Surface};
use glium::glutin::{WindowBuilder};
use cam::Camera;
use programs::Programs;
use keymap::{Keymap, Key};
use std::rc::Rc;
use viewer::Viewer;
use app::{Message as AppMessage};
use std::sync::mpsc::Sender;
use std::time::{Instant, Duration};
use hand::Hand;

pub struct Model {
    display: Rc<Display>,
    programs: Programs,
    keymap: Keymap,
    camera: Camera,
    app: Sender<AppMessage>,
    hand: Hand,
    viewer: Viewer,
}

pub enum Message {
    Quit,
    MoveCamera(Direction),
    MoveHand(Direction),
    ResetCamera,
    EmitAnimationFrame,
}

pub fn run(viewer: Viewer, app: Sender<AppMessage>) {
    let mut model = init(viewer, app);
    loop {
        let message = draw(&model);
        match update(message, model) {
            None => return,
            Some(next_model) => model = next_model,
        }
    }
}

pub fn init(viewer: Viewer, app: Sender<AppMessage>) -> Model {
    use programs::HandType;
    let display: Rc<Display> = Rc::new(WindowBuilder::new().with_title("vr counter")
                                                           .with_depth_buffer(24)
                                                           .build_glium()
                                                           .unwrap());
    Model {
        app: app,
        display: display.clone(),
        programs: Programs::new(display, viewer.clone(), HandType::Keyboard),
        keymap: Keymap::init(),
        camera: Camera::start(),
        hand: Default::default(),
        viewer: viewer.clone(),
    }
}

pub fn update(message: Message, mut model: Model) -> Option<Model> {
    use app::Message as AppMessage;
    match message {
        Message::Quit => None,
        Message::ResetCamera => Some(model.with_camera(Camera::start())),
        Message::MoveCamera(direction) => {
            let camera = get_camera(&model, direction);
            Some(model.with_camera(camera))
        },
        Message::EmitAnimationFrame => {
            model.app.send(AppMessage::EmitAnimationFrame).unwrap_or(());
            Some(model)
        },
        Message::MoveHand(direction) => {
            const STEP: f32 = 0.05;
            let (dx, dy, dz) = match direction {
                Direction::Up => (0.0, STEP, 0.0),
                Direction::Down => (0.0, -STEP, 0.0),
                Direction::Left => (-STEP, 0.0, 0.0),
                Direction::Right => (STEP, 0.0, 0.0),
                Direction::Far => (0.0, 0.0, -STEP),
                Direction::Near => (0.0, 0.0, STEP),
            };
            let offset = model.hand.offset.shift(dx, dy, dz);
            model.hand.offset = offset;
            model.viewer.set_hand(model.hand);
            model.app.send(AppMessage::SetHand(model.hand)).unwrap();
            Some(model)
        },
    }
}

fn get_camera(model: &Model, direction: Direction) -> Camera {
    match direction {
        Direction::Up => model.camera.move_up(),
        Direction::Down => model.camera.move_down(),
        Direction::Left => model.camera.move_left(),
        Direction::Right => model.camera.move_right(),
        Direction::Near => model.camera.move_near(),
        Direction::Far => model.camera.move_far(),
    }
}

pub fn draw(model: &Model) -> Message {
    let mut target = model.display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    let (view, perspective) = model.camera.get_view_and_projection(&target);
    model.programs.draw(&mut target, &view, &perspective);
    target.finish().unwrap();

    let frame_instant = Instant::now();
    let frame_duration = Duration::from_millis(300);

    let mut message_option: Option<Message> = None;
    while message_option.is_none() {
        for glutin_event in model.display.poll_events() {
            if let Some(key) = model.keymap.key_for_event(&glutin_event) {
                message_option = message_option_from_key(key);
                if message_option.is_some() {
                    break;
                }
            }
        }
        if Instant::now().duration_since(frame_instant) > frame_duration {
            message_option = Some(Message::EmitAnimationFrame);
        }
    }
    return message_option.unwrap();
}

impl Model {
    pub fn with_camera(self, camera: Camera) -> Self {
        Model {
            display: self.display,
            programs: self.programs,
            keymap: self.keymap,
            camera: camera,
            app: self.app,
            hand: self.hand,
            viewer: self.viewer,
        }
    }
}

fn message_option_from_key(key: Key) -> Option<Message> {
    match key {
        Key::LookUp => Some(Message::MoveCamera(Direction::Up)),
        Key::LookDown => Some(Message::MoveCamera(Direction::Down)),
        Key::LookRight => Some(Message::MoveCamera(Direction::Right)),
        Key::LookLeft => Some(Message::MoveCamera(Direction::Left)),
        Key::LookFar => Some(Message::MoveCamera(Direction::Far)),
        Key::LookNear => Some(Message::MoveCamera(Direction::Near)),
        Key::ResetLook => Some(Message::ResetCamera),
        Key::Quit => Some(Message::Quit),
        Key::H => Some(Message::MoveHand(Direction::Left)),
        Key::J => Some(Message::MoveHand(Direction::Down)),
        Key::K => Some(Message::MoveHand(Direction::Up)),
        Key::L => Some(Message::MoveHand(Direction::Right)),
        Key::RBracket => Some(Message::MoveHand(Direction::Far)),
        Key::LBracket => Some(Message::MoveHand(Direction::Near)),
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Near,
    Far
}
