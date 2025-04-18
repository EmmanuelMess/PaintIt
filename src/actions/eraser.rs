use crate::actions::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum EraserSize {
    #[default]
    SizeOne,
    SizeTwo,
    SizeThree,
    SizeFour,
}

impl EraserSize {
    fn width(self) -> f32 {
        match self {
            EraserSize::SizeOne => 8f32,
            EraserSize::SizeTwo => 16f32,
            EraserSize::SizeThree => 32f32,
            EraserSize::SizeFour => 64f32,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct EraserState {
    old_mouse_position: Option<CanvasVector2>,
    mouse_position: Option<CanvasVector2>,
    size: EraserSize,
}

impl UpdateExecuteAction for EraserState {
    fn update_pressed(&mut self, user_state: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        self.old_mouse_position = self.mouse_position;
        self.mouse_position = Option::from(user_state.to_canvas(user_state.mouse_position));
    }

    fn update_unpressed(&mut self, _: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        self.old_mouse_position = None;
        self.mouse_position = None;
    }

    fn update_after_draw(&mut self, _: &UserState) {}

    fn draw(&mut self, image: &mut Image) -> bool {
        let color = Color::new(0,0,0,0);

        match (self.old_mouse_position, self.mouse_position) {
            (None, None) => {
                // Nothing
                false
            }
            (None, Some(mouse_position)) => {
                image.draw_rectangle(
                    (mouse_position.0.x - self.size.width()/2f32) as i32,
                    (mouse_position.0.y - self.size.width()/2f32) as i32,
                    self.size.width() as i32,
                    self.size.width() as i32,
                    color
                );
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
                    image.draw_rectangle(
                        (x0 - self.size.width()/2f32) as i32,
                        (y0 - self.size.width()/2f32) as i32,
                        self.size.width() as i32,
                        self.size.width() as i32,
                        color
                    );

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

    fn draw_state(&self, _: &UserState, _: &mut RaylibDrawHandle, _: &RaylibThread) {}

    fn get_color(&self) -> Option<Color> {
        None
    }
}
