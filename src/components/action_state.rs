#[derive(Clone, PartialEq, Debug)]
pub enum ActionState {
    None,
    Moving { right: bool, left: bool, up: bool, down: bool },
    Attacking,
}

impl Default for ActionState {
    fn default() -> Self {
        ActionState::None
    }
}
