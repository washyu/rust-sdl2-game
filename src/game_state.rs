use crate::components::*;
use crate::components::ai::Ai;
use crate::entity_factory::EntityFactory;
use crate::systems::ai_system::AiSystem;
use crate::systems::health_system::HealthSystem;
use crate::systems::render_system::RenderSystem;
use crate::systems::*;
use crate::asset_manager::AssetManager;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct GameState<'a> {
    pub entities: Vec<Entity>,
    pub positions: Vec<Position>,
    pub healths: Vec<Health>,
    pub input_bindings: Vec<InputBindings>,
    pub textures: Vec<Vec<Texture<'a>>>,
    pub animations: Vec<Animation>,
    pub ais: Vec<Ai>,
    pub action_states: Vec<ActionState>,
    pub tilemap: Option<Tilemap<'a>>,
    pub tile_types: HashMap<TileId, TileType>,
    pub camera_x: i32,
    pub camera_y: i32,
    input_system: InputSystem,  // Keep the InputSystem instance
}

impl<'a> GameState<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut entity_factory = match EntityFactory::new(texture_creator) {
            Ok(factory) => factory,
            Err(e) => {
                eprintln!("Failed to create entity factory: {}", e);
                panic!("Could not initialize entity factory");
            }
        };
        
        let mut entities = Vec::new();
        let mut positions = Vec::new();
        let mut healths = Vec::new();
        let mut textures = Vec::new();
        let mut animations = Vec::new();
        let mut input_bindings = Vec::new();
        let mut ais = Vec::new();
        let mut action_states = Vec::new();
        
        // Create player
        match entity_factory.create_entity("player", 100.0, 100.0) {
            Ok((entity, position, health, entity_textures, animation, binding, ai, action_state)) => {
                entities.push(entity);
                positions.push(position);
                healths.push(health);
                textures.push(entity_textures);
                animations.push(animation);
                input_bindings.push(binding);
                ais.push(ai);
                action_states.push(action_state);
            },
            Err(e) => eprintln!("Failed to create player: {}", e),
        }
        
        // Create enemy
        match entity_factory.create_entity("goblin", 200.0, 200.0) {
            Ok((entity, position, health, entity_textures, animation, binding, ai, action_state)) => {
                entities.push(entity);
                positions.push(position);
                healths.push(health);
                textures.push(entity_textures);
                animations.push(animation);
                input_bindings.push(binding);
                ais.push(ai);
                action_states.push(action_state);
            },
            Err(e) => eprintln!("Failed to create goblin: {}", e),
        }
        
        // Initialize tile types
        let mut tile_types = HashMap::new();
        tile_types.insert(TileId(0), TileType::Empty);
        tile_types.insert(TileId(1), TileType::Floor);
        tile_types.insert(TileId(2), TileType::Wall);
        tile_types.insert(TileId(3), TileType::Water);
        tile_types.insert(TileId(4), TileType::Lava);
        

        let tilesets_dir = std::path::Path::new("assets/tilesets");
        if !tilesets_dir.exists() {
            println!("WARNING: Tilesets directory doesn't exist at {:?}", tilesets_dir.to_path_buf());
        }   

        // Load tilemap with tileset
        let mut asset_manager = AssetManager::new();
        let tilemap = match asset_manager.load_tilemap_with_tileset(
            texture_creator, 
            "levels/level1.csv", 
            "tilesets/Texture/TX Tileset Grass.png", 
            32, 
            32
        ) {
            Ok(map) => Some(map),
            Err(e) => {
                eprintln!("Failed to load tilemap: {}", e);
                None
            }
        };
        
        // Create InputSystem with enough capacity
        let input_system = InputSystem::new(200);  // 200 should be enough for all keys
        
        GameState {
            ais,
            healths,
            entities,
            positions,
            input_bindings,
            textures,
            animations,
            action_states,
            tilemap,
            tile_types,
            camera_x: 0,
            camera_y: 0,
            input_system,
        }
    }
    
    
    pub fn update(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState, delta_time: f32) {
        // Get player input actions
        let player_action_states = self.input_system.run(
            &self.entities[0..1],  // Just process player entity (index 0)
            &self.input_bindings[0..1], 
            keyboard_state
        );
        
        // Update player action state
        if !self.animations[0].is_attack_in_progress { // Only update if not attacking
            self.action_states[0] = player_action_states[0].clone();
        }
        
        // Debug what state was assigned to player
        match &self.action_states[0] {
            ActionState::Moving { right, left, up, down } => {
                println!("Player has movement state: right={}, left={}, up={}, down={}", 
                         right, left, up, down);
            },
            ActionState::Attacking => println!("Player is attacking"),
            ActionState::None => {} // Don't log None state
        }
        
        // Update player movement
        MovementSystem::run(
            &self.entities[0..1],  // Just player
            &mut self.positions[0..1], 
            &self.action_states[0..1],
            self.tilemap.as_ref(), 
            Some(&self.tile_types)
        );
        
        // Update AI for non-player entities
        AiSystem::run(
            &self.entities[1..],  // Skip player (starts at index 1)
            &self.positions[1..], 
            &mut self.ais[1..], 
            &mut self.action_states[1..]
        );
        
        // Update movement for non-player entities
        MovementSystem::run(
            &self.entities[1..], 
            &mut self.positions[1..], 
            &self.action_states[1..],
            self.tilemap.as_ref(), 
            Some(&self.tile_types)
        );
        
        // Update health
        HealthSystem::update(&self.entities, &mut self.healths, delta_time);
        
        // Update animation states based on action states
        self.update_animations();
        
        // Update animation frames
        for animation in &mut self.animations {
            animation.update(16);
        }
        
        // Update the camera to follow the player
        if let Some(player_pos) = self.positions.get(0).cloned() {
            self.update_camera(&player_pos);
        }
        
        // Debug animation progress
        for (i, animation) in self.animations.iter().enumerate() {
            if animation.is_attack_in_progress {
                println!("Entity {} attack animation: frame {}/{}", i, 
                         animation.current_frame, 
                         match animation.state {
                             AnimationState::Attack => animation.attack_frames,
                             _ => 0
                         });
            }
        }
    }
    
    fn update_animations(&mut self) {
        for (i, action_state) in self.action_states.iter().enumerate() {
            if let Some(animation) = self.animations.get_mut(i) {
                // Handle attack initiation
                if let ActionState::Attacking = action_state {
                    if !animation.is_attack_in_progress {  // Remove parentheses
                        println!("Starting attack animation for entity {}", i);
                        animation.state = AnimationState::Attack;
                        animation.current_frame = 0;
                        animation.is_attack_in_progress = true;
                    }
                } 
                // Only change to other states if not attacking
                else if !animation.is_attack_in_progress {  // Remove parentheses
                    let new_state = match action_state {
                        ActionState::Moving { right: true, .. } |
                        ActionState::Moving { left: true, .. } |
                        ActionState::Moving { up: true, .. } |
                        ActionState::Moving { down: true, .. } => AnimationState::Walk,
                        _ => AnimationState::Idle,
                    };
                    
                    // Only update if state changes
                    if animation.state != new_state {  // Remove parentheses
                        animation.state = new_state;
                        animation.current_frame = 0;
                    }
                }
            }
        }
    }
    
    pub fn update_camera(&mut self, player_pos: &Position) {
        // Screen dimensions
        let screen_width = 800; // Your screen width
        let screen_height = 600; // Your screen height
        
        // Center camera on player with boundaries
        if let Some(tilemap) = &self.tilemap {
            // Calculate max camera position (so we don't show beyond the map)
            let max_camera_x = (tilemap.width * tilemap.tile_size as usize) as i32 - screen_width;
            let max_camera_y = (tilemap.height * tilemap.tile_size as usize) as i32 - screen_height;
            
            // Center on player
            self.camera_x = (player_pos.x as i32 - screen_width / 2).max(0).min(max_camera_x);
            self.camera_y = (player_pos.y as i32 - screen_height / 2).max(0).min(max_camera_y);
        }
    }
    
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        RenderSystem::render(
            canvas,
            &self.entities,
            &self.textures,
            &self.animations,
            &self.positions,
            self.camera_x,
            self.camera_y,
            self.tilemap.as_ref()
        );
    }
}