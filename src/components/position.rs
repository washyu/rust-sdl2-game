// src/components/position.rs
#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub facing_right: bool, // Add this field to track facing direction
}

impl Position {
    pub fn new(x: f32, y: f32, facing_right: bool) -> Self {
        Self { x, y, facing_right }
    }
}