#[derive(Clone, PartialEq)]
pub enum ActionState {
    Moving {right: bool, left: bool, up: bool, down: bool},
    Attacking,
    None,
}

// Add this implementation for Default
impl Default for ActionState {
    fn default() -> Self {
        ActionState::Moving { right: false, left: false, up: false, down: false }
    }
}

// Add for compatibility with your movement system
impl ActionState {
    pub const MoveUp: Self = ActionState::Moving { right: false, left: false, up: true, down: false };
    pub const MoveDown: Self = ActionState::Moving { right: false, left: false, up: false, down: true };
    pub const MoveLeft: Self = ActionState::Moving { right: false, left: true, up: false, down: false };
    pub const MoveRight: Self = ActionState::Moving { right: true, left: false, up: false, down: false };
}