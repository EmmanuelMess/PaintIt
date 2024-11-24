use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::prelude::Image;
use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::UserState;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct PencilState {
    last_frame_mouse_position_in_canvas: Option<Vector2>,
    mouse_position_in_canvas: Option<Vector2>,
    color: Color,
}

impl UpdateExecuteAction for PencilState {
    fn update_pressed(&mut self, user_state: UserState) {
        self.last_frame_mouse_position_in_canvas = self.mouse_position_in_canvas;
        self.mouse_position_in_canvas = Option::from(user_state.to_canvas(user_state.mouse_position));
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, user_state: UserState) {
        self.last_frame_mouse_position_in_canvas = None;
        self.mouse_position_in_canvas = None;
        self.color = user_state.current_colors[0];
    }

    fn update_after_draw(&mut self, _: UserState) {}

    fn draw(&self, image: &mut Image) -> bool {
        match (self.last_frame_mouse_position_in_canvas, self.mouse_position_in_canvas) {
            (None, None) => {
                // Nothing
                false
            }
            (None, Some(mousePositionInCanvas)) => {
                image.draw_pixel_v(mousePositionInCanvas, self.color);
                true
            }
            (Some(lastFrameMousePositionInCanvas), Some(mousePositionInCanvas)) => {
                image.draw_line_v(lastFrameMousePositionInCanvas, mousePositionInCanvas,
                                  self.color);
                true
            }
            (Some(_), None) => { panic!("Older position is newer than new position!"); }
        }
    }

    fn draw_state(&self, _: &mut RaylibDrawHandle, _: UserState) {}
}
