use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum BrushSize {
    One,
    #[default]
    Two,
    Three,
}

impl BrushSize {
    fn width(self) -> f32 {
        match self {
            BrushSize::One => 4f32,
            BrushSize::Two => 8f32,
            BrushSize::Three => 16f32,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum BrushType {
    #[default]
    Circle,
    Square,
    ForwardLine,
    BackwardLine,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct BrushState {
    old_mouse_position: Option<CanvasVector2>,
    mouse_position: Option<CanvasVector2>,
    color: Color,
    size: BrushSize,
    brush_type: BrushType,
}

impl BrushState {
    fn draw_shape(self, image: &mut Image, position: Vector2) {
        match self.brush_type {
            BrushType::Circle => {
                image.draw_circle(
                    position.x as i32,
                    position.y as i32,
                    (self.size.width() / 2f32) as i32,
                    self.color
                );
            }
            BrushType::Square => {
                image.draw_rectangle(
                    (position.x - self.size.width()/2f32) as i32,
                    (position.y - self.size.width()/2f32) as i32,
                    self.size.width() as i32,
                    self.size.width() as i32,
                    self.color
                );
            }
            BrushType::ForwardLine => {
                panic!("Forward line not implemented!");
            }
            BrushType::BackwardLine => {
                panic!("Backward line not implemented!");
            }
        }
    }
}

impl UpdateExecuteAction for BrushState {
    fn update_pressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.old_mouse_position = self.mouse_position;
        self.mouse_position = Option::from(user_state.to_canvas(user_state.mouse_position));
        self.size = user_state.brush_size;
        self.brush_type = user_state.brush_type;
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.old_mouse_position = None;
        self.mouse_position = None;
    }

    fn update_after_draw(&mut self, user_state: &UserState) {}

    fn draw(&mut self, image: &mut Image) -> bool {
        match (self.old_mouse_position, self.mouse_position) {
            (None, None) => {
                // Nothing
                false
            }
            (None, Some(mouse_position)) => {
                self.draw_shape(image, mouse_position.0);
                true
            }
            (Some(old_frame_mouse_position), Some(mouse_position)) => {
                // Bresenham's line algorithm from https://rosettacode.org/wiki/Bitmap/Bresenham's_line_algorithm

                let mut x0 = old_frame_mouse_position.0.x;
                let mut y0 = old_frame_mouse_position.0.y;
                let x1 = mouse_position.0.x;
                let y1 = mouse_position.0.y;

                let dx = (x1-x0).abs();
                let sx = if x0 < x1 { 1f32 } else { -1f32 };
                let dy = (y1-y0).abs();
                let sy = if y0 < y1 { 1f32 } else { -1f32 };

                let mut err = if dx>dy { dx/2f32 } else { -dy/2f32 };

                loop {
                    self.draw_shape(image, Vector2 { x:x0, y:y0 });

                    if x0 == x1 && y0 == y1 {
                        break;
                    }

                    let e2 = err;
                    if e2 > -dx {
                        err -= dy;
                        x0 += sx;
                    }
                    if e2 < dy {
                        err += dx;
                        y0 += sy;
                    }
                }
                true
            }
            (Some(_), None) => { panic!("Older position is newer than new position!"); }
        }
    }

    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, thread: &RaylibThread) {}

    fn get_color(&self) -> Option<Color> {
        None
    }
}
