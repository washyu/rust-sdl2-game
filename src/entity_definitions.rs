
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EntityDefinition {
    pub entity_type: String,
    pub health: u32,
    pub max_health: u32,
    pub speed: f32,
    pub textures: HashMap<String, String>,
    pub animation_frames: HashMap<String, usize>,
    pub ai_type: Option<String>,
    pub collision: CollisionInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CollisionInfo {
    pub width: f32,
    pub height: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityDefinitions {
    pub entities: HashMap<String, EntityDefinition>,
}