use raylib::color::Color;
use raylib::math::Vector2;
use raylib::texture::{Image};
use crate::actions::brush::{BrushSize, BrushType};
use crate::actions::spray::SpraySize;

/// Vector2 in the Canvas coordinate system
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct CanvasVector2(pub Vector2);

/// Vector2 in the Window coordinate system
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct WindowVector2(pub Vector2);

#[derive(Clone)]
pub struct UserState {
    pub current_colors: [Color; 2],
    pub mouse_position: WindowVector2,
    pub canvas_position: WindowVector2,
    pub spray_size: SpraySize,
    pub brush_size: BrushSize,
    pub brush_type: BrushType,
    pub canvas_image: Image,
}

impl UserState {
    pub fn to_canvas(&self, vector: WindowVector2) -> CanvasVector2 {
        CanvasVector2(vector.0 - self.canvas_position.0)
    }

    pub fn to_window(&self, vector: CanvasVector2) -> WindowVector2 {
        WindowVector2(vector.0 + self.canvas_position.0)
    }
}