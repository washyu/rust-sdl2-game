use std::collections::HashMap;
use std::sync::Arc;
use sdl2::render::{TextureCreator, Texture as SdlTexture};
use sdl2::video::WindowContext;
use std::path::Path;
use crate::components::texture::Texture;
use crate::components::tilemap::Tilemap;

pub struct ResourceManager<'a> {
    textures: HashMap<String, Arc<Texture<'a>>>,
    tilesets: HashMap<String, Arc<Texture<'a>>>,
    tilemaps: HashMap<String, Arc<Tilemap<'a>>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> Self {
        ResourceManager {
            textures: HashMap::new(),
            tilesets: HashMap::new(),
            tilemaps: HashMap::new(),
        }
    }
    
    pub fn get_texture(&mut self, creator: &'a TextureCreator<WindowContext>, path: &str) -> Result<Arc<Texture<'a>>, String> {
        // Check cache first
        if let Some(texture) = self.textures.get(path) {
            return Ok(Arc::clone(texture));
        }
        
        // Prepend assets/ to the path if needed
        let full_path = if !path.starts_with("assets/") {
            format!("assets/{}", path)
        } else {
            path.to_string()
        };
        
        // Load the texture
        let texture = Arc::new(Texture::new(creator, &full_path)?);
        
        // Cache and return
        self.textures.insert(path.to_string(), Arc::clone(&texture));
        Ok(texture)
    }
    
    pub fn get_tilemap(&mut self, 
                    creator: &'a TextureCreator<WindowContext>, 
                    map_path: &str,
                    tileset_path: &str,
                    tile_width: u32,
                    tile_height: u32) -> Result<Arc<Tilemap<'a>>, String> {
        
        let key = format!("{}-{}", map_path, tileset_path);
        
        // Check cache first
        if let Some(tilemap) = self.tilemaps.get(&key) {
            return Ok(Arc::clone(tilemap));
        }
        
        // Get the tileset texture
        let tileset = self.get_texture(creator, tileset_path)?;
        
        // Create the tilemap using the new load_from_file method
        let full_path = format!("assets/{}", map_path);
        let tilemap = Arc::new(Tilemap::load_from_file(
            &full_path, 
            &tileset,
            tile_width as usize
        )?);
        
        // Cache and return
        self.tilemaps.insert(key, Arc::clone(&tilemap));
        Ok(tilemap)
    }
}