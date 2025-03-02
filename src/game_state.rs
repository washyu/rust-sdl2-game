use crate::components::*;
use crate::systems::*;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct GameState<'a> {
    pub entities: Vec<Entity>,
    pub positions: Vec<Position>,
    pub healths: Vec<Health>,
    pub input_bindings: Vec<InputBindings>,
    pub ais: Vec<AI>,
    pub textures: Vec<Vec<Texture<'a>>>,
    pub animations: Vec<Animation>,
    pub action_states: Vec<ActionState>,
}

impl<'a> GameState<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let entities = vec![Entity(0), Entity(1)];
        
        let positions = vec![
            Position { x: 100.0, y: 100.0, facing_right: true },
            Position { x: 200.0, y: 200.0, facing_right: true },
        ];
        
        let healths = vec![
            Health::new(100, 100),
            Health::new(50, 50),
        ];
        
        let input_bindings = vec![
            InputBindings::new(vec![
                (sdl2::keyboard::Scancode::W, GameAction::MoveUp),
                (sdl2::keyboard::Scancode::A, GameAction::MoveLeft),
                (sdl2::keyboard::Scancode::S, GameAction::MoveDown),
                (sdl2::keyboard::Scancode::D, GameAction::MoveRight),
            ]),
            InputBindings::default(),
        ];
        
        let ais = vec![
            AI { mode: AIMode::Patrol },
            AI { mode: AIMode::ChasePlayer },
        ];
        
        let textures = Self::load_textures(texture_creator);
        
        let animations = vec![
            Animation::new(6, AnimationState::Idle),
            Animation::new(1, AnimationState::Idle),
        ];
        
        let action_states = vec![ActionState::None; entities.len()];
        
        GameState {
            entities,
            positions,
            healths,
            input_bindings,
            ais,
            textures,
            animations,
            action_states,
        }
    }
    
    fn load_textures(texture_creator: &'a TextureCreator<WindowContext>) -> Vec<Vec<Texture<'a>>> {
        let mut textures: Vec<Vec<Texture>> = vec![Vec::new(), Vec::new()];
        
        // Player textures
        let player_textures = [
            "assets/Characters(100x100)/Soldier/Soldier with shadows/Soldier-Idle.png",
            "assets/Characters(100x100)/Soldier/Soldier with shadows/Soldier-Walk.png",
            "assets/Characters(100x100)/Soldier/Soldier with shadows/Soldier-Attack01.png",
            "assets/Characters(100x100)/Soldier/Soldier with shadows/Soldier-Attack02.png",
            "assets/Characters(100x100)/Soldier/Soldier with shadows/Soldier-Attack03.png",
            "assets/Characters(100x100)/Soldier/Soldier with shadows/Soldier-Hurt.png",
            "assets/Characters(100x100)/Soldier/Soldier with shadows/Soldier-Death.png",
        ];
        
        for path in &player_textures {
            match Texture::new(texture_creator, path) {
                Ok(tex) => textures[0].push(tex),
                Err(e) => eprintln!("Failed to load texture {}: {}", path, e),
            }
        }
        
        // Monster textures
        let monster_textures = ["assets/monster.png"];
        
        for path in &monster_textures {
            match Texture::new(texture_creator, path) {
                Ok(tex) => textures[1].push(tex),
                Err(e) => eprintln!("Failed to load texture {}: {}", path, e),
            }
        }
        
        textures
    }
    
    pub fn update(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
        // Process input
        self.action_states = InputSystem::run(&self.entities, &self.input_bindings, keyboard_state);
        
        // Update movement
        MovementSystem::run(&self.entities, &mut self.positions, &self.action_states);
        
        // Update animations based on actions
        self.update_animations();
        
        // Update animation frames
        for animation in &mut self.animations {
            animation.update(16);
        }
    }
    
    fn update_animations(&mut self) {
        for (i, action_state) in self.action_states.iter().enumerate() {
            if let Some(animation) = self.animations.get_mut(i) {
                // Handle attack initiation
                if let ActionState::Attacking = action_state {
                    if !animation.is_attack_in_progress {
                        animation.state = AnimationState::Attack;
                        animation.current_frame = 0;
                        animation.is_attack_in_progress = true;
                    }
                } 
                // Only change to other states if not attacking
                else if !animation.is_attack_in_progress {
                    let new_state = match action_state {
                        ActionState::Moving { right: true, .. } |
                        ActionState::Moving { left: true, .. } |
                        ActionState::Moving { up: true, .. } |
                        ActionState::Moving { down: true, .. } => AnimationState::Walk,
                        _ => AnimationState::Idle,
                    };
                    
                    // Only update if state changes
                    if animation.state != new_state {
                        animation.state = new_state;
                        animation.current_frame = 0;
                    }
                }
            }
        }
    }
    
    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        RenderSystem::render(canvas, &self.entities, &self.textures, &self.animations, &self.positions);
    }
}