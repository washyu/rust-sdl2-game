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
        for (i, _entity) in entities.iter().enumerate() {
            // Debug what action state we have for this entity
            println!("MovementSystem processing entity {} with state: {:?}", i, action_states[i]);
            
            // Store the current position in case we need to revert
            let current_pos = positions[i];
            let mut moved = false;
            
            // Update position based on action
            match &action_states[i] {
                ActionState::Moving { right, left, up, down } => {
                    // Debug which keys are active
                    println!("Moving with right={}, left={}, up={}, down={}", right, left, up, down);
                    
                    if *right {
                        positions[i].x += 5.0;
                        positions[i].facing_right = true;
                        moved = true;
                    }
                    if *left {
                        positions[i].x -= 5.0;
                        positions[i].facing_right = false;
                        moved = true;
                    }
                    if *up {
                        positions[i].y -= 5.0;
                        moved = true;
                    }
                    if *down {
                        positions[i].y += 5.0;
                        moved = true;
                    }
                    
                    if moved {
                        println!("Entity {} moved to ({}, {})", i, positions[i].x, positions[i].y);
                    }
                },
                ActionState::Attacking => {
                    // Handle attacking logic
                },
                ActionState::None => {
                    // No action
                }
            }
            
            // Skip collision detection for now to test if basic movement works
            if moved && tilemap.is_none() {
                continue;
            }
            
            if moved {
                // Check for collision with tilemap
                if let (Some(tilemap), Some(tile_types)) = (tilemap, tile_types) {
                    // IMPROVED: Smaller collision box and better positioning
                    
                    // Get sprite dimensions
                    let sprite_width = 100.0 * 2.0;
                    let sprite_height = 100.0 * 2.0;
                    
                    // MUCH smaller collision box
                    let collision_width = 20.0;   // Reduced from 40.0
                    let collision_height = 10.0;  // Reduced from 20.0
                    
                    // Position at bottom center and raised a bit
                    let collision_offset_x = (sprite_width - collision_width) / 2.0;
                    let collision_offset_y = sprite_height - collision_height - 30.0; // Raised more
                    
                    // Calculate collision box boundaries
                    let collision_left = positions[i].x + collision_offset_x;
                    let collision_top = positions[i].y + collision_offset_y;
                    let collision_right = collision_left + collision_width;
                    let collision_bottom = collision_top + collision_height;
                    
                    // Convert to tile coordinates
                    let tile_left = (collision_left / tilemap.tile_size as f32) as usize;
                    let tile_top = (collision_top / tilemap.tile_size as f32) as usize;
                    let tile_right = (collision_right / tilemap.tile_size as f32) as usize;
                    let tile_bottom = (collision_bottom / tilemap.tile_size as f32) as usize;
                    
                    // Debug output - but limit it to avoid spam
                    if i == 0 { // Only show for player
                        println!("Collision box: ({},{}) to ({},{}) - tiles: ({},{}) to ({},{})", 
                                 collision_left, collision_top, collision_right, collision_bottom,
                                 tile_left, tile_top, tile_right, tile_bottom);
                    }
                    
                    // Check collision with all tiles the entity's collision box overlaps
                    let mut collides = false;
                    
                    // Check all tiles in the collision box
                    for tile_y in tile_top..=tile_bottom {
                        for tile_x in tile_left..=tile_right {
                            if tile_x < tilemap.width && tile_y < tilemap.height {
                                if let Some(tile_id) = tilemap.get_tile(tile_x, tile_y) {
                                    if let Some(tile_type) = tile_types.get(&tile_id) {
                                        if tile_type == &TileType::Wall {
                                            collides = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if collides {
                            break;
                        }
                    }
                    
                    if collides {
                        // Collision detected, revert the move
                        positions[i] = current_pos;
                        
                        if i == 0 { // Only show for player to avoid spam
                            println!("Collision detected! Reverting to ({}, {})", current_pos.x, current_pos.y);
                        }
                    }
                }
            }
        }
    }
}