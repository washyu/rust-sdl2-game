mod components;
mod systems;
mod game_state;
mod entity_factory;
mod entity_definitions;
mod asset_manager;

use sdl2::event::Event;
use std::time::{Instant, Duration};
use game_state::GameState;

fn main() -> Result<(), String> {
    // debug to see where the program is running.
    println!("Working directory: {:?}", std::env::current_dir().unwrap());

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
    
    let mut last_frame_time = Instant::now();

    'running: loop {
        // Calculate delta time
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;

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
        game.update(&keyboard_state, delta_time);
        
        // Render
        game.render(&mut canvas);
        
        // Cap frame rate
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}
