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
        let mut keys = HashMap::new();
        
        // Arrow keys
        keys.insert(Scancode::Right, GameAction::MoveRight);
        keys.insert(Scancode::Left, GameAction::MoveLeft);
        keys.insert(Scancode::Up, GameAction::MoveUp);
        keys.insert(Scancode::Down, GameAction::MoveDown);
        
        // WASD keys
        keys.insert(Scancode::D, GameAction::MoveRight);
        keys.insert(Scancode::A, GameAction::MoveLeft);
        keys.insert(Scancode::W, GameAction::MoveUp);
        keys.insert(Scancode::S, GameAction::MoveDown);
        
        // Attack key
        keys.insert(Scancode::Space, GameAction::Attack);
        
        InputBindings { keys }
    }
}
