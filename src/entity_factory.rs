use crate::components::ai::Ai;
use crate::entity_definitions::EntityDefinitions;
use crate::components::*;
use crate::resource_manager::ResourceManager;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::keyboard::Scancode;
use std::sync::Arc;


pub struct EntityFactory<'a> {
    definitions: EntityDefinitions,
    resource_manager: ResourceManager<'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
    next_entity_id: u32,
}

impl<'a> EntityFactory<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> Result<Self, String> {
        // Load entity definitions from RON file
        let ron_str = std::fs::read_to_string("assets/entities.ron")
            .map_err(|e| format!("Failed to read entity definitions: {}", e))?;
            
        let definitions: EntityDefinitions = ron::from_str(&ron_str)
            .map_err(|e| format!("Failed to parse entity definitions: {}", e))?;
            
        Ok(EntityFactory {
            definitions,
            resource_manager: ResourceManager::new(),
            texture_creator,
            next_entity_id: 0,
        })
    }
    
    pub fn next_id(&mut self) -> u32 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }
    
    pub fn create_entity(&mut self, 
        entity_name: &str, 
        x: f32, 
        y: f32
    ) -> Result<(Entity, Position, Health, Vec<Arc<Texture<'a>>>, Animation, InputBindings, Ai, ActionState), String> {
        // Get entity definition
        let definition = self.definitions.entities.get(entity_name)
            .ok_or_else(|| format!("Entity definition not found: {}", entity_name))?.clone();
        
        // Create entity with ID
        let entity_id = self.next_id();
        let entity = Entity(entity_id as usize);
        
        // Create position
        let position = Position::new(x, y, true); // Assuming the entity is facing right by default
        
        // Create health
        let health = Health::new(definition.health, definition.max_health);
        
        // Load textures for all animation types
        let texture_types = vec!["idle", "walk", "attack"];
        let mut entity_textures = Vec::new();
        
        for &anim_type in &texture_types {
            if let Some(texture_path) = definition.textures.get(anim_type) {
                match self.resource_manager.get_texture(self.texture_creator, texture_path) {
                    Ok(texture) => entity_textures.push(texture),
                    Err(e) => eprintln!("Failed to load texture {}: {}", texture_path, e),
                }
            }
        }
        
        // Create animation
        let idle_frames = definition.animation_frames.get("idle").copied().unwrap_or(1);
        let walk_frames = definition.animation_frames.get("walk").copied().unwrap_or(1);
        let attack_frames = definition.animation_frames.get("attack").copied().unwrap_or(1);

        let animation = Animation::new(
            AnimationState::Idle, 
            idle_frames,
            walk_frames,
            attack_frames
        );
        
        // Create input bindings
        let input_bindings = if entity_name == "player" {
            InputBindings::new(vec![
                // Arrow keys
                (Scancode::Right, GameAction::MoveRight),
                (Scancode::Left, GameAction::MoveLeft),
                (Scancode::Up, GameAction::MoveUp),
                (Scancode::Down, GameAction::MoveDown),
                // WASD keys - add these
                (Scancode::D, GameAction::MoveRight),
                (Scancode::A, GameAction::MoveLeft),
                (Scancode::W, GameAction::MoveUp),
                (Scancode::S, GameAction::MoveDown),
                // Attack key
                (Scancode::Space, GameAction::Attack),
            ])
        } else {
            InputBindings::default()
        };
        
        // Create AI
        let ai = match definition.ai_type.as_deref() {
            Some("patrol") => Ai { behavior: AiState::Patrol { current_waypoint: 0, waypoints: vec![] }},
            Some("chase") => Ai { behavior: AiState::Chase { target_entity: 0, detection_range: 200.0, attack_range: 50.0 }},
            _ => Ai { behavior: AiState::Idle }, // Changed 'mode' to 'behavior'
        };
        
        // Create action state
        let action_state = ActionState::None;
        
        Ok((entity, position, health, entity_textures, animation, input_bindings, ai, action_state))
    }
}