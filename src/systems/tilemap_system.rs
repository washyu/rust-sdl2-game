use crate::components::Tilemap;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct TilemapRenderSystem;

impl TilemapRenderSystem {
    pub fn render<'a>(canvas: &mut Canvas<Window>, tilemap: &Tilemap<'a>, camera_x: i32, camera_y: i32) {
        // Skip if no tileset is loaded
        let tileset = match &tilemap.tileset {
            Some(ts) => ts,
            _none => return,
        };
        
        // Calculate visible area (same as before)
        let screen_width = canvas.viewport().width() as i32;
        let screen_height = canvas.viewport().height() as i32;
        
        let start_x = (camera_x / tilemap.tile_size as i32).max(0) as usize;
        let start_y = (camera_y / tilemap.tile_size as i32).max(0) as usize;
        let end_x = ((camera_x + screen_width) / tilemap.tile_size as i32 + 1).min(tilemap.width as i32) as usize;
        let end_y = ((camera_y + screen_height) / tilemap.tile_size as i32 + 1).min(tilemap.height as i32) as usize;
        
        // Render visible tiles
        for y in start_y..end_y {
            for x in start_x..end_x {
                if y < tilemap.tiles.len() && x < tilemap.tiles[y].len() {
                    let tile_id = tilemap.tiles[y][x].0;
                    
                    // Only render non-empty tiles
                    if tile_id > 0 {
                        if let Some(src_rect) = tileset.get_tile_rect(tile_id - 1) { // Adjust for 0-based indexing
                            let dest_rect = sdl2::rect::Rect::new(
                                (x * tilemap.tile_size as usize) as i32 - camera_x,
                                (y * tilemap.tile_size as usize) as i32 - camera_y,
                                tilemap.tile_size,
                                tilemap.tile_size
                            );
                            
                            canvas.copy(&tileset.texture.handle, Some(src_rect), Some(dest_rect))
                                .unwrap_or_else(|e| {
                                    eprintln!("Error rendering tile at ({}, {}): {}", x, y, e);
                                });
                        }
                    }
                }
            }
        }
    }
}