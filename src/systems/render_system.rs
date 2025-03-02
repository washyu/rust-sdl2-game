use crate::components::texture::Texture;  // Import Texture directly
use crate::components::{Entity, Position, Animation, AnimationState};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub struct RenderSystem;

impl RenderSystem {
    pub fn render(
        canvas: &mut Canvas<Window>, 
        entities: &[Entity], 
        textures: &[Vec<Texture>],  // Change to accept Vec<Vec<Texture>>
        animations: &[Animation], 
        positions: &[Position]
    ) {
        // Clear and prepare the canvas
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Render each entity
        for (i, _) in entities.iter().enumerate() {
            if let (Some(entity_textures), Some(animation), Some(position)) = (textures.get(i), animations.get(i), positions.get(i)) {
                // Get the right texture and frame based on animation state
                let texture_index = match animation.state {
                    AnimationState::Idle => 0,    // Soldier-Idle.png is at index 0
                    AnimationState::Walk => 1,    // Soldier-Walk.png is at index 1
                    AnimationState::Attack => 2,  // Soldier-Attack01.png is at index 2
                    AnimationState::Hurt => 5,    // Soldier-Hurt.png is at index 5
                    AnimationState::Death => 6,   // Soldier-Death.png is at index 6
                };

                // Each state has different frame counts - add this
                let frame_count = match animation.state {
                    AnimationState::Idle => 6,
                    AnimationState::Walk => 8,
                    AnimationState::Hurt => 4,
                    AnimationState::Death => 4,
                    AnimationState::Attack => 9,
                };

                // Get the right texture for the current entity and animation state
                if let Some(texture) = entity_textures.get(texture_index) {
                    // Calculate correct frame within the sprite sheet
                    let current_frame = animation.current_frame % frame_count;
                    
                    // Define frame dimensions
                    let frame_width = 100;
                    let frame_height = 100;
                    
                    let x_offset = frame_width * current_frame;
                    let clip_rect = sdl2::rect::Rect::new(
                        x_offset as i32, 0, 
                        frame_width as u32, frame_height as u32
                    );
                    
                    let scale_factor = 2.0;
                    let dest_rect = sdl2::rect::Rect::new(
                        position.x as i32,
                        position.y as i32,
                        (frame_width as f32 * scale_factor) as u32,
                        (frame_height as f32 * scale_factor) as u32
                    );
                    
                    // Replace the canvas.copy call with copy_ex for flip support
                    let flip_horizontal = !position.facing_right;

                    // Use copy_ex instead of copy to support flipping
                    canvas.copy_ex(
                        &texture.handle, 
                        Some(clip_rect), 
                        Some(dest_rect),
                        0.0,             // Rotation angle (0 = no rotation)
                        None,            // Center of rotation (None = center of dest_rect)
                        flip_horizontal, // Flip horizontally based on facing direction
                        false            // Don't flip vertically
                    ).unwrap();
                }
            }
        }
        
        canvas.present();
    }
}
