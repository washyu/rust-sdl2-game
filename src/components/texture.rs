use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::surface::Surface;

pub struct Texture<'a> {
    pub path: String,
    pub handle: sdl2::render::Texture<'a>,
}

impl<'a> Texture<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, path: &str) -> Result<Self, String> {
        // Load surface first
        let surface = Surface::load_bmp(path)
            .or_else(|_| sdl2::image::LoadSurface::from_file(path))
            .map_err(|e| e.to_string())?;
        
        // Create texture from surface
        let texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        
        Ok(Texture { handle: texture, path: path.to_string() })
    }

    // src/components/texture.rs - Fix the load method
    pub fn load(path: &str, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let surface = Surface::load_bmp(path).map_err(|e| e.to_string())?;
        // Or if using SDL2_image:
        // let surface = sdl2::image::LoadSurface::load_surface(path).map_err(|e| e.to_string())?;
        
        let texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        
        Ok(Texture { handle: texture, path: path.to_string() })
    }
}