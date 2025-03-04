use crate::components::ai::Ai;
use crate::entity_definitions::EntityDefinitions;
use crate::components::*;
use crate::asset_manager::AssetManager;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;


pub struct EntityFactory<'a> {
    definitions: EntityDefinitions,
    asset_manager: AssetManager<'a>,
    texture_creator: &'a TextureCreator<WindowContext>,
    next_entity_id: u32,
}

impl<'a> EntityFactory<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>
    ) -> Result<Self, String> {
        // Load entity definitions from RON file
        let ron_str = std::fs::read_to_string("entities.ron")
            .map_err(|e| format!("Failed to read entity definitions: {}", e))?;
            
        let definitions: EntityDefinitions = ron::from_str(&ron_str)
            .map_err(|e| format!("Failed to parse entity definitions: {}", e))?;
            
        Ok(EntityFactory {
            definitions,
            asset_manager: AssetManager::new(),
            texture_creator,
            next_entity_id: 0,
        })
    }
    
    pub fn next_id(&mut self) -> u32 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }
    
    pub fn create_entity(
        &mut self, 
        entity_name: &str, 
        x: f32, 
        y: f32
    ) -> Result<(Entity, Position, Health, Vec<Texture<'a>>, Animation, InputBindings, Ai, ActionState), String> {
        // Create entity with ID
        let entity_id = self.next_id();
        let entity = Entity(entity_id as usize);

        let definition = self.definitions.entities.get(entity_name)
            .ok_or_else(|| format!("Entity definition not found: {}", entity_name))?;
        
        // Create components
        let position = Position { x, y, facing_right: true };
        let health = Health::new(definition.health, definition.health);
        
        // Load textures based on definition
        let mut textures = Vec::new();
        let texture_types = ["idle", "walk", "attack"];
        
        for &anim_type in &texture_types {
            if let Some(texture_path) = definition.textures.get(anim_type) {
                match Texture::new(self.texture_creator, texture_path) {
                    Ok(texture) => textures.push(texture),
                    Err(e) => eprintln!("Failed to load texture {}: {}", texture_path, e),
                }
            }
        }
        
        // Get animation frame counts from definition
        let idle_frames = definition.animation_frames.get("idle").copied().unwrap_or(1);
        let walk_frames = definition.animation_frames.get("walk").copied().unwrap_or(1);
        let attack_frames = definition.animation_frames.get("attack").copied().unwrap_or(1);

        // Create animation with frames from definition
        let animation = Animation::new(
            AnimationState::Idle, 
            idle_frames,
            walk_frames,
            attack_frames
        );
        
        // Create input bindings (only for player)
        let input_bindings = if entity_name == "player" {
            InputBindings::new(vec![
                (sdl2::keyboard::Scancode::W, GameAction::MoveUp),
                (sdl2::keyboard::Scancode::A, GameAction::MoveLeft),
                (sdl2::keyboard::Scancode::S, GameAction::MoveDown),
                (sdl2::keyboard::Scancode::D, GameAction::MoveRight),
                (sdl2::keyboard::Scancode::Space, GameAction::Attack),
            ])
        } else {
            InputBindings::default()
        };
        
        // Add AI component based on entity type
        let ai = match definition.ai_type.as_deref() {
            Some("patrol") => Ai { behavior: AiState::Patrol { current_waypoint: 0, waypoints: vec![] }},
            Some("chase") => Ai { behavior: AiState::Chase { target_entity: 0, detection_range: 200.0, attack_range: 50.0 }},
            _ => Ai { behavior: AiState::Idle },
        };
        
        // Initial action state
        let action_state = ActionState::None;
        
        // Return entity and components
        Ok((entity, position, health, textures, animation, input_bindings, ai, action_state))
    }
}