use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use crate::components::{InputBindings, ActionState, Entity, GameAction};

pub struct InputSystem;

impl InputSystem {
    pub fn run(entities: &[Entity], bindings: &[InputBindings], keyboard: &KeyboardState) -> Vec<ActionState> {
        let mut action_states = vec![ActionState::default(); entities.len()];
        
        // Process input for each entity with input bindings
        for (i, _entity) in entities.iter().enumerate() {
            if let Some(binding) = bindings.get(i) {
                if keyboard.is_scancode_pressed(Scancode::Space) {
                    action_states[i] = ActionState::Attacking;
                    continue;  // Skip checking other keys if attacking
                }
                for (&key, &action) in &binding.keys {
                    if keyboard.is_scancode_pressed(key) {
                        action_states[i] = match action {
                            GameAction::MoveUp => ActionState::MoveUp,
                            GameAction::MoveDown => ActionState::MoveDown,
                            GameAction::MoveLeft => ActionState::MoveLeft,
                            GameAction::MoveRight => ActionState::MoveRight,
                            _ => ActionState::default(),
                        };
                    }
                }
            }
        }
        
        action_states
    }
}