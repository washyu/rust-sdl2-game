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
        
        // Add debug output to see what's happening
        println!("Input system running, keyboard state detected: {}", 
                 if keyboard.is_scancode_pressed(sdl2::keyboard::Scancode::Right) { "right pressed" } else { "right not pressed" });
        
        for (i, _) in entities.iter().enumerate() {
            if let Some(bindings) = input_bindings.get(i) {
                let mut right = false;
                let mut left = false;
                let mut up = false;
                let mut down = false;
                let mut attack = false;
                
                // Check each key binding - CHANGE THIS PART
                for (scancode, action) in &bindings.keys { // Use keys not bindings
                    if keyboard.is_scancode_pressed(*scancode) {
                        match action {
                            GameAction::MoveRight => right = true,
                            GameAction::MoveLeft => left = true,
                            GameAction::MoveUp => up = true,
                            GameAction::MoveDown => down = true,
                            GameAction::Attack => attack = true,
                        }
                    }
                }
                
                // Debug output for player entity
                if i == 0 {
                    println!("Player input: right={}, left={}, up={}, down={}, attack={}", 
                             right, left, up, down, attack);
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