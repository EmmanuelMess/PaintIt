use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Image;
use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::UserState;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct LineState {
    start: Option<Vector2>,
    end: Option<Vector2>,
    draw_now: bool,
    color: Color,
}

impl UpdateExecuteAction for LineState {
    fn update_pressed(&mut self, user_state: UserState) {
        if self.start == None {
            self.start = Option::from(user_state.to_canvas(user_state.mouse_position));
        } else {
            self.end = Option::from(user_state.to_canvas(user_state.mouse_position));
        }
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, _: UserState) {
        if self.start != None && self.end != None {
            self.draw_now = true;
        }
    }

    fn update_after_draw(&mut self, _: UserState) {
        if self.draw_now {
            self.start = None;
            self.end = None;
            self.draw_now = false;
        }
    }

    fn draw(&mut self, image: &mut Image) -> bool {
        if self.draw_now {
            image.draw_line_v(self.start.unwrap(), self.end.unwrap(), self.color);
            return true;
        }

        return false;
    }

    fn draw_state(&self, handle: &mut RaylibDrawHandle, user_state: UserState) {
        if !(self.start != None && self.end != None) {
            return;
        }

        let p0 = user_state.to_window(self.start.unwrap());
        let p1 = user_state.to_window(self.end.unwrap());

        handle.draw_line_v(p0, p1, self.color);
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
