mod components;
mod systems;
mod game_state;
mod entity_factory;
mod asset_manager;

use sdl2::event::Event;
use std::time::Duration;
use game_state::GameState;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    // Initialize SDL2_image
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::PNG)?;
    
    let window = video_subsystem.window("Sprite Animation", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    
    let mut event_pump = sdl_context.event_pump()?;
    
    // Create game state
    let mut game = GameState::new(&texture_creator);
    
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        
        // Get keyboard state
        let keyboard_state = event_pump.keyboard_state();
        
        // Update game state
        game.update(&keyboard_state);
        
        // Render
        game.render(&mut canvas);
        
        // Cap frame rate
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}
