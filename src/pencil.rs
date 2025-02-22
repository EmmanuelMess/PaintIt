use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct PencilState {
    old_mouse_position: Option<CanvasVector2>,
    mouse_position: Option<CanvasVector2>,
    color: Color,
}

impl UpdateExecuteAction for PencilState {
    fn update_pressed(&mut self, user_state: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        self.old_mouse_position = self.mouse_position;
        self.mouse_position = Option::from(user_state.to_canvas(user_state.mouse_position));
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, user_state: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        self.old_mouse_position = None;
        self.mouse_position = None;
        self.color = user_state.current_colors[0];
    }

    fn update_after_draw(&mut self, _: &UserState) {}

    fn draw(&mut self, image: &mut Image) -> bool {
        match (self.old_mouse_position, self.mouse_position) {
            (None, None) => {
                // Nothing
                false
            }
            (None, Some(mouse_position_in_canvas)) => {
                image.draw_pixel_v(mouse_position_in_canvas.0, self.color);
                true
            }
            (Some(old_frame_mouse_position_in_canvas), Some(mouse_position_in_canvas)) => {
                image.draw_line_v(old_frame_mouse_position_in_canvas.0, mouse_position_in_canvas.0,
                                  self.color);
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
