use sakura::PressLabel;
use hand::Hand;

pub enum UserEvent {
    Stop,
    EmitAnimationFrame,
    SetHand(Hand),
    Press(PressLabel),
    Release(PressLabel),
}