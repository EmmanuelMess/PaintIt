use raylib::color::Color;
use raylib::math::Vector2;
use crate::brush::{BrushSize, BrushType};
use crate::spray::SpraySize;

#[derive(Clone, Copy)]
pub struct UserState {
    pub current_colors: [Color; 2],
    pub mouse_position: Vector2,
    pub canvas_position: Vector2,
    pub spray_size: SpraySize,
    pub brush_size: BrushSize,
    pub brush_type: BrushType,
}

impl UserState {
    pub fn to_canvas(self, vector: Vector2) -> Vector2 {
        vector - self.canvas_position
    }

    pub fn to_window(self, vector: Vector2) -> Vector2 {
        vector + self.canvas_position
    }
}