use crate::components::{ActionState, Entity, Position};

pub struct MovementSystem;

impl MovementSystem {
    pub fn run(entities: &[Entity], positions: &mut [Position], action_states: &[ActionState]) {
        for (i, _entity) in entities.iter().enumerate() {
            match action_states[i] {
                ActionState::Moving { right: true, .. } => {
                    positions[i].x += 2.0;
                    positions[i].facing_right = true; // Set facing right when moving right
                },
                ActionState::Moving { left: true, .. } => {
                    positions[i].x -= 2.0;
                    positions[i].facing_right = false; // Set facing left when moving left
                },
                ActionState::Moving { up: true, .. } => positions[i].y -= 2.0, // No change to facing for up/down
                ActionState::Moving { down: true, .. } => positions[i].y += 2.0,
                _ => {}
            }
        }
    }
}