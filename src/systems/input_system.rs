use crate::components::{InputBindings, ActionState, Entity, GameAction};

pub struct InputSystem {
    previous_keys: Vec<bool>, // Track previous frame's key states
}

impl InputSystem {
    pub fn new(num_keys: usize) -> Self {
        Self {
            previous_keys: vec![false; num_keys],
        }
    }

    pub fn run(&mut self, entities: &[Entity], input_bindings: &[InputBindings], keyboard: &sdl2::keyboard::KeyboardState) -> Vec<ActionState> {
        let mut action_states = vec![ActionState::None; entities.len()];
        
        for (i, _) in entities.iter().enumerate() {
            if let Some(bindings) = input_bindings.get(i) {
                let mut right = false;
                let mut left = false;
                let mut up = false;
                let mut down = false;
                let mut attack = false;
                
                // Check each binding
                for (j, (&scancode, &action)) in bindings.keys.iter().enumerate() {
                    let key_pressed = keyboard.is_scancode_pressed(scancode);
                    let key_just_pressed = key_pressed && !self.previous_keys.get(j).copied().unwrap_or(false);
                    
                    // Update previous key state
                    if let Some(prev) = self.previous_keys.get_mut(j) {
                        *prev = key_pressed;
                    } else {
                        self.previous_keys.push(key_pressed);
                    }
                    
                    match action {
                        GameAction::MoveRight => right = key_pressed,
                        GameAction::MoveLeft => left = key_pressed,
                        GameAction::MoveUp => up = key_pressed,
                        GameAction::MoveDown => down = key_pressed,
                        GameAction::Attack => {
                            // Only set attack on key press, not key hold
                            if key_just_pressed {
                                attack = true;
                                println!("Attack key JUST pressed");
                            }
                        },
                    }
                }
                
                // Set appropriate action state
                if attack {
                    action_states[i] = ActionState::Attacking;
                } else if right || left || up || down {
                    action_states[i] = ActionState::Moving { right, left, up, down };
                }
            }
        }
        
        action_states
    }
}