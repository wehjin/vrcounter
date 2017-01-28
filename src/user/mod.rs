use sakura::PressLabel;
use sakura::AsciiPoint;
use hand::Hand;

pub enum UserEvent {
    Stop,
    EmitAnimationFrame,
    SetHand(Hand),
    Press(PressLabel),
    Release(PressLabel),
    Preview(Option<AsciiPoint>),
}