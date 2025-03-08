use crate::components::texture::Texture;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc; // Remove the duplicate import
use std::collections::HashMap;

pub struct Tilemap<'a> {
    pub width: usize,
    pub height: usize,
    pub tile_size: u32,
    pub tiles: Vec<Vec<TileId>>,
    pub tileset: Option<Tileset<'a>>,
    pub tile_textures: HashMap<TileId, Texture<'a>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct TileId(pub u32);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Floor,
    Wall,
    Water,
    Lava,
}

impl<'a> Tilemap<'a> {
    pub fn new(width: usize, height: usize, tile_size: u32) -> Self {
        // Create empty tilemap
        let tiles = vec![vec![TileId(0); width]; height];
        
        Tilemap {
            width,
            height,
            tile_size,
            tiles,
            tileset: None,
            tile_textures: HashMap::new(),
        }
    }
    
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TileId> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y][x])
        } else {
            None
        }
    }
    
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileId) {
        if x < self.width && y < self.height {
            self.tiles[y][x] = tile;
        }
    }
    
    pub fn is_solid(&self, x: usize, y: usize, tile_types: &HashMap<TileId, TileType>) -> bool {
        match self.get_tile(x, y) {
            Some(tile_id) => {
                match tile_types.get(tile_id) {
                    Some(tile_type) => *tile_type == TileType::Wall || *tile_type == TileType::Lava,
                    _ => false,
                }
            },
            _ => true, // Consider out-of-bounds as solid
        }
    }

    pub fn load_from_file(
        file_path: &str, 
        tileset_texture: &Arc<Texture<'a>>, 
        tile_size: usize
    ) -> Result<Self, String> {
        // Open and read the CSV file
        let file = File::open(file_path)
            .map_err(|e| format!("Failed to open tilemap file {}: {}", file_path, e))?;
        
        let reader = BufReader::new(file);
        let mut tiles = Vec::new();
        let mut width = 0;
        
        // Parse each line
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            if line.trim().is_empty() {
                continue;
            }
            
            let row: Vec<TileId> = line
                .split(',')
                .filter_map(|s| {
                    s.trim().parse::<u32>().ok().map(TileId)
                })
                .collect();
                
            if width == 0 {
                width = row.len();
            } else if row.len() != width {
                return Err(format!("Inconsistent row length in tilemap file"));
            }
            
            tiles.push(row);
        }
        
        if tiles.is_empty() {
            return Err("Empty tilemap file".to_string());
        }
        
        let height = tiles.len();
        
        // Create tilemap
        let mut tilemap = Tilemap {
            width,
            height,
            tile_size: tile_size as u32,
            tiles,
            tileset: None,
            tile_textures: HashMap::new(),
        };
        
        // Create tileset from the provided texture
        // Fix this to match the definition of Tileset::new
        let tileset = Tileset::new(
            tileset_texture.clone(), 
            tile_size as u32, 
            tile_size as u32 // Adding the missing tile_height parameter
        );
        tilemap.tileset = Some(tileset);
        
        Ok(tilemap)
    }
}

pub struct Tileset<'a> {
    pub texture: Arc<Texture<'a>>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
}

impl<'a> Tileset<'a> {
    pub fn new(texture: Arc<Texture<'a>>, tile_width: u32, tile_height: u32) -> Self {
        let texture_query = texture.handle.query();
        let columns = texture_query.width / tile_width;
        let rows = texture_query.height / tile_height;
        
        Tileset {
            texture,
            tile_width,
            tile_height,
            columns,
            rows,
        }
    }
    
    pub fn get_tile_rect(&self, tile_id: u32) -> Option<sdl2::rect::Rect> {
        if tile_id >= self.columns * self.rows {
            return None;
        }
        
        let column = tile_id % self.columns;
        let row = tile_id / self.columns;
        
        Some(sdl2::rect::Rect::new(
            (column * self.tile_width) as i32,
            (row * self.tile_height) as i32,
            self.tile_width,
            self.tile_height,
        ))
    }
}