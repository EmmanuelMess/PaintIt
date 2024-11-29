use std::path::absolute;
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::prelude::Image;
use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::UserState;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct ColorPickerState {
    position: Option<Vector2>,
    color: Color,
}

impl UpdateExecuteAction for ColorPickerState {
    fn update_pressed(&mut self, user_state: UserState) {
        self.position = Option::from(user_state.to_canvas(user_state.mouse_position));
    }

    fn update_unpressed(&mut self, _: UserState) {}

    fn update_after_draw(&mut self, _: UserState) {}

    fn draw(&mut self, image: &mut Image) -> bool {
        if let Some(position)  = self.position {
            self.color = image.get_color(position.x as i32, position.y as i32);
        }
        false
    }

    fn draw_state(&self, _: &mut RaylibDrawHandle, _: UserState) {}

    fn get_color(&self) -> Option<Color> {
        Option::from(self.color)
    }
}
