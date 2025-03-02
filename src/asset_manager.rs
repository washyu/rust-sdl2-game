use crate::components::texture::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct AssetManager<'a> {
    textures: HashMap<String, Texture<'a>>,
}

impl<'a> AssetManager<'a> {
    pub fn new() -> Self {
        AssetManager {
            textures: HashMap::new(),
        }
    }
    
    pub fn load_texture(&mut self, creator: &'a TextureCreator<WindowContext>, path: &str) -> Result<&Texture<'a>, String> {
        if !self.textures.contains_key(path) {
            let texture = Texture::new(creator, path)?;
            self.textures.insert(path.to_string(), texture);
        }
        
        Ok(self.textures.get(path).unwrap())
    }
    
    pub fn get_texture(&self, path: &str) -> Option<&Texture<'a>> {
        self.textures.get(path)
    }
}