use crate::components::{ActionState, Entity, Position, Tilemap, TileType, TileId};
use std::collections::HashMap;

pub struct MovementSystem;

impl MovementSystem {
    pub fn run(
        entities: &[Entity], 
        positions: &mut [Position], 
        action_states: &[ActionState],
        tilemap: Option<&Tilemap>,
        tile_types: Option<&HashMap<TileId, TileType>>
    ) {
        for (i, _) in entities.iter().enumerate() {
            if let Some(position) = positions.get_mut(i) {
                if let Some(action_state) = action_states.get(i) {
                    println!("MovementSystem processing entity {} with state: {:?}", i, action_state);
                    
                    match action_state {
                        ActionState::Moving { right, left, up, down } => {
                            // Calculate movement vector
                            let mut dx = 0.0;
                            let mut dy = 0.0;
                            
                            if *right { dx += position.speed; }
                            if *left { dx -= position.speed; }
                            if *up { dy -= position.speed; }
                            if *down { dy += position.speed; }
                            
                            // Debug output
                            if dx != 0.0 || dy != 0.0 {
                                println!("Moving entity {} by ({}, {})", i, dx, dy);
                            }
                            
                            // Update position
                            position.x += dx;
                            position.y += dy;
                            
                            // Update facing direction
                            if dx != 0.0 {
                                position.facing_right = dx > 0.0;
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}