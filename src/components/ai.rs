// In components/ai.rs or similar file
pub enum AiState {
    Idle,
    Patrol { waypoints: Vec<(f32, f32)>, current_waypoint: usize },
    Chase { target_entity: usize, detection_range: f32, attack_range: f32 },
}

pub struct Ai {
    // Define the fields for the Ai struct
    pub behavior: AiState,
}
