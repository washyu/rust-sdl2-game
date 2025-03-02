pub struct Position {
    pub x: f32,
    pub y: f32,
    pub facing_right: bool, // Add this field to track facing direction
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, facing_right: true } // Default facing right
    }
}