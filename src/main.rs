mod components;
mod systems;
mod game_state;
mod entity_factory;
mod entity_definitions;
mod asset_manager;
mod resource_manager;

use sdl2::{event::Event, keyboard::Scancode};
use std::time::{Instant, Duration};
use game_state::GameState;
use sdl2::pixels::Color;

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
    
    // Add this right after creating the game state
    let player_count = game.entities.len();
    println!("Loaded {} entities", player_count);
    if player_count > 0 {
        println!("Player position: ({}, {})", game.positions[0].x, game.positions[0].y);
    }
    
    let mut last_frame_time = Instant::now();

    'running: loop {
        // Calculate delta time
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;

        // Process events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                _ => {}
            }
        }

        // Get keyboard state
        let keyboard_state = event_pump.keyboard_state();

        // Debug output to check if SDL2 is detecting ANY keys at all
        if keyboard_state.is_scancode_pressed(Scancode::Right) ||
            keyboard_state.is_scancode_pressed(Scancode::Left) ||
            keyboard_state.is_scancode_pressed(Scancode::Up) ||
            keyboard_state.is_scancode_pressed(Scancode::Down) ||
            keyboard_state.is_scancode_pressed(Scancode::Space) {
            println!("SDL2 DETECTED KEY PRESS: Right={}, Left={}, Up={}, Down={}, Space={}",
            keyboard_state.is_scancode_pressed(Scancode::Right),
            keyboard_state.is_scancode_pressed(Scancode::Left),
            keyboard_state.is_scancode_pressed(Scancode::Up),
            keyboard_state.is_scancode_pressed(Scancode::Down),
            keyboard_state.is_scancode_pressed(Scancode::Space));
        }
        
        // ADD NEW DEBUG CODE HERE
        if keyboard_state.is_scancode_pressed(Scancode::W) ||
           keyboard_state.is_scancode_pressed(Scancode::A) ||
           keyboard_state.is_scancode_pressed(Scancode::S) ||
           keyboard_state.is_scancode_pressed(Scancode::D) {
            println!("SDL2 DETECTED WASD: W={}, A={}, S={}, D={}",
                keyboard_state.is_scancode_pressed(Scancode::W),
                keyboard_state.is_scancode_pressed(Scancode::A),
                keyboard_state.is_scancode_pressed(Scancode::S),
                keyboard_state.is_scancode_pressed(Scancode::D));
        }
        
        // Update game state
        game.update(&keyboard_state, delta_time);
        
        // Clear the screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Render the game
        game.render(&mut canvas);
        
        // Present the frame
        canvas.present();
        
        // Cap frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}
