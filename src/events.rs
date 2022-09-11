pub struct ControlEvent {
    pub action: ControlAction,
}

#[derive(Clone, PartialEq)]
pub enum ControlAction {
    Forward,
    Backwards,
    Left,
    Right,
}
