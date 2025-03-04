use crate::components::texture::Texture;
use std::collections::HashMap;

pub struct Tilemap<'a> {
    pub width: usize,
    pub height: usize,
    pub tile_size: u32,
    pub tiles: Vec<Vec<TileId>>,
    pub tileset: Option<Tileset<'a>>,
    pub tile_textures: HashMap<TileId, Texture<'a>>,  // Add this field
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
            tile_textures: HashMap::new(),  // Initialize the new field
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
                    _none => false,
                }
            },
            _none => true, // Consider out-of-bounds as solid
        }
    }
}

pub struct Tileset<'a> {
    pub texture: Texture<'a>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
}

impl<'a> Tileset<'a> {
    pub fn new(texture: Texture<'a>, tile_width: u32, tile_height: u32) -> Self {
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