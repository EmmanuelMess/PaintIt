use crate::actions::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct ColorPickerState {
    position: Option<CanvasVector2>,
    color: Color,
}

impl UpdateExecuteAction for ColorPickerState {
    fn update_pressed(&mut self, user_state: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        self.position = Option::from(user_state.to_canvas(user_state.mouse_position));
    }

    fn update_unpressed(&mut self, _: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {}

    fn update_after_draw(&mut self, _: &UserState) {}

    fn draw(&mut self, image: &mut Image) -> bool {
        if let Some(position)  = self.position {
            self.color = image.get_color(position.0.x as i32, position.0.y as i32);
        }
        false
    }

    fn draw_state(&self, _: &UserState, _: &mut RaylibDrawHandle, _: &RaylibThread) {}

    fn get_color(&self) -> Option<Color> {
        Option::from(self.color)
    }
}
