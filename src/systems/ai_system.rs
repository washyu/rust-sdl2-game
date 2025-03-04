use crate::components::{AiState, Entity, Position};
use crate::components::ai::Ai;  // Changed from AiState to Ai
use crate::components::ActionState;

// systems/ai_system.rs
pub struct AiSystem;

impl AiSystem {
    pub fn run(
        entities: &[Entity],
        positions: &[Position],
        ais: &mut [Ai],  // Changed from AiState to Ai
        action_states: &mut [ActionState],
    ) {
        let _ = entities;
        for (i, ai) in ais.iter_mut().enumerate() {
            match &mut ai.behavior {  // Access the AiState through the behavior field
                AiState::Idle => {
                    // Random chance to start patrolling
                    action_states[i] = ActionState::None;
                },
                AiState::Patrol { waypoints, current_waypoint } => {
                    // Skip if no waypoints are defined
                    if waypoints.is_empty() {
                        action_states[i] = ActionState::None;
                        continue;
                    }
                    
                    // Get the current target waypoint
                    let (target_x, target_y) = waypoints[*current_waypoint];
                    
                    // Calculate direction to waypoint
                    let dx = target_x - positions[i].x;
                    let dy = target_y - positions[i].y;
                    let distance = (dx*dx + dy*dy).sqrt();
                    
                    // If we've reached the waypoint (within a small threshold)
                    if distance < 5.0 {
                        // Move to next waypoint
                        *current_waypoint = (*current_waypoint + 1) % waypoints.len();
                        action_states[i] = ActionState::None;
                    } else {
                        // Move toward waypoint
                        let right = dx > 0.0;
                        let left = dx < 0.0;
                        let up = dy < 0.0;
                        let down = dy > 0.0;
                        action_states[i] = ActionState::Moving { right, left, up, down };
                    }
                },
                AiState::Chase { target_entity, detection_range, attack_range } => {
                    // Chase behavior implementation
                    if *target_entity < positions.len() {
                        let dx = positions[*target_entity].x - positions[i].x;
                        let dy = positions[*target_entity].y - positions[i].y;
                        let distance = (dx*dx + dy*dy).sqrt();
                        
                        if distance < *attack_range {
                            action_states[i] = ActionState::Attacking;
                        } else if distance < *detection_range {
                            let right = dx > 0.0;
                            let left = dx < 0.0;
                            let up = dy < 0.0;
                            let down = dy > 0.0;
                            action_states[i] = ActionState::Moving { right, left, up, down };
                        }
                    }
                }
            }
        }
    }
}