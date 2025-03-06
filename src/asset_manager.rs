use crate::components::tilemap::Tileset;
use crate::components::{Tilemap, TileId, Texture};
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct AssetManager<'a> {
    textures: HashMap<String, Texture<'a>>,
}

impl<'a> AssetManager<'a> {
    pub fn new() -> Self {
        AssetManager {
            textures: HashMap::new(),
        }
    }
    
    pub fn load_texture(&mut self, creator: &'a TextureCreator<WindowContext>, path: &str) 
        -> Result<Texture<'a>, String> 
    {
        // Check if we have already loaded this texture
        if let Some(texture) = self.textures.get(path) {
            // Here's the tricky part - we need to clone the texture somehow
            // But SDL textures can't be cloned...
            return Err("Texture system needs redesign - can't clone SDL textures".to_string());
        }
        
        // Load the texture
        let full_path = if !path.starts_with("assets/") {
            format!("assets/{}", path)
        } else {
            path.to_string()
        };
        
        Texture::new(creator, &full_path)
    }
    
    pub fn load_tilemap(&mut self, 
                      creator: &'a TextureCreator<WindowContext>, 
                      path: &str,
                      tile_size: u32) -> Result<Tilemap<'a>, String> {
        // Load the CSV file
        let file = File::open(path).map_err(|e| format!("Failed to open file {}: {}", path, e))?;
        let reader = BufReader::new(file);
        
        let mut tiles = Vec::new();
        let mut width = 0;
        
        // Parse the CSV file
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            if line.trim().is_empty() {
                continue;
            }
            
            let row: Vec<TileId> = line.split(',')
                .filter(|s| !s.is_empty())
                .map(|s| TileId(s.trim().parse::<u32>().unwrap_or(0)))
                .collect();
            
            if width == 0 {
                width = row.len();
            } else if row.len() != width {
                return Err(format!("Inconsistent row width in CSV file: expected {}, got {}", width, row.len()));
            }
            
            tiles.push(row);
        }
        
        if tiles.is_empty() {
            return Err("CSV file contains no valid tile data".to_string());
        }
        
        let height = tiles.len();
        println!("Loaded tilemap: {}x{}", width, height);
        
        // Create the tilemap
        let mut tilemap = Tilemap::new(width, height, tile_size);
        tilemap.tiles = tiles;
        
        // Load tile textures
        let texture_paths = [
            (TileId(0), "assets/tiles/empty.png"),
            (TileId(1), "assets/tiles/floor.png"),
            (TileId(2), "assets/tiles/wall.png"),
            (TileId(3), "assets/tiles/water.png"),
            (TileId(4), "assets/tiles/lava.png"),
        ];
        
        for (id, path) in texture_paths.iter() {
            match Texture::new(creator, path) {
                Ok(texture) => { tilemap.tile_textures.insert(*id, texture); },
                Err(e) => println!("Warning: Failed to load texture {}: {}", path, e),
            }
        }
        
        Ok(tilemap)
    }
    
    pub fn load_tileset(&self, texture_creator: &'a TextureCreator<WindowContext>, path: &str, tile_width: u32, tile_height: u32) -> Result<Tileset<'a>, String> {
        let texture = Texture::new(texture_creator, path)?;
        Ok(Tileset::new(texture, tile_width, tile_height))
    }
    
    pub fn load_tilemap_with_tileset(&mut self, 
                      creator: &'a TextureCreator<WindowContext>, 
                      map_path: &str,
                      tileset_path: &str,
                      tile_width: u32,
                      tile_height: u32) -> Result<Tilemap<'a>, String> {
        // Load the CSV file
        let file = File::open(map_path).map_err(|e| format!("Failed to open file {}: {}", map_path, e))?;
        let reader = BufReader::new(file);
        
        let mut tiles = Vec::new();
        let mut width = 0;
        
        // Parse the CSV file
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            if line.trim().is_empty() {
                continue;
            }
            
            let row: Vec<TileId> = line.split(',')
                .filter(|s| !s.is_empty())
                .map(|s| TileId(s.trim().parse::<u32>().unwrap_or(0)))
                .collect();
            
            if width == 0 {
                width = row.len();
            } else if row.len() != width {
                return Err(format!("Inconsistent row width in CSV file: expected {}, got {}", width, row.len()));
            }
            
            tiles.push(row);
        }
        
        if tiles.is_empty() {
            return Err("CSV file contains no valid tile data".to_string());
        }
        
        let height = tiles.len();
        println!("Loaded tilemap with tileset: {}x{}", width, height);
        
        // Create the tilemap
        let mut tilemap = Tilemap::new(width, height, tile_width);
        tilemap.tiles = tiles;
        
        // Load the tileset
        match self.load_tileset(creator, tileset_path, tile_width, tile_height) {
            Ok(tileset) => tilemap.tileset = Some(tileset),
            Err(e) => return Err(format!("Failed to load tileset: {}", e)),
        }
        
        Ok(tilemap)
    }
}