use std::collections::HashMap;
use sdl2::keyboard::Scancode;
use crate::components::game_action::GameAction;

pub struct InputBindings {
    pub keys: HashMap<Scancode, GameAction>,
}

impl InputBindings {
    pub fn new(bindings: Vec<(Scancode, GameAction)>) -> Self {
        let keys = bindings.into_iter().collect();
        InputBindings { keys }
    }
}

impl Default for InputBindings {
    fn default() -> Self {
        Self { keys: HashMap::new() }
    }
}