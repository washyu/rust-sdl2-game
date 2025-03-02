use crate::components::*;
use sdl2::keyboard::Scancode;

pub struct EntityFactory;

impl EntityFactory {
    pub fn create_player(entity_id: usize, x: f32, y: f32) -> (Entity, Position, Health, InputBindings) {
        let entity = Entity(entity_id);
        
        let position = Position {
            x,
            y,
            facing_right: true,
        };
        
        let health = Health::new(100, 100);
        
        let input_bindings = InputBindings::new(vec![
            (Scancode::W, GameAction::MoveUp),
            (Scancode::A, GameAction::MoveLeft),
            (Scancode::S, GameAction::MoveDown),
            (Scancode::D, GameAction::MoveRight),
            (Scancode::Space, GameAction::Attack),
        ]);
        
        (entity, position, health, input_bindings)
    }
    
    pub fn create_monster(entity_id: usize, x: f32, y: f32) -> (Entity, Position, Health, AI) {
        let entity = Entity(entity_id);
        
        let position = Position {
            x,
            y,
            facing_right: true,
        };
        
        let health = Health::new(50, 50);
        
        let ai = AI {
            mode: AIMode::ChasePlayer,
        };
        
        (entity, position, health, ai)
    }
}