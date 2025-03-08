use crate::actions::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct LineState {
    start: Option<CanvasVector2>,
    end: Option<CanvasVector2>,
    draw_now: bool,
    color: Color,
}

impl UpdateExecuteAction for LineState {
    fn update_pressed(&mut self, user_state: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        if self.start == None {
            self.start = Option::from(user_state.to_canvas(user_state.mouse_position));
        } else {
            self.end = Option::from(user_state.to_canvas(user_state.mouse_position));
        }
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, _: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        if self.start != None && self.end != None {
            self.draw_now = true;
        }
    }

    fn update_after_draw(&mut self, _: &UserState) {
        if self.draw_now {
            self.start = None;
            self.end = None;
            self.draw_now = false;
        }
    }

    fn draw(&mut self, image: &mut Image) -> bool {
        if self.draw_now {
            image.draw_line_v(self.start.unwrap().0, self.end.unwrap().0, self.color);
            return true;
        }

        return false;
    }

    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, _: &RaylibThread) {
        if !(self.start != None && self.end != None) {
            return;
        }

        let p0 = user_state.to_window(self.start.unwrap());
        let p1 = user_state.to_window(self.end.unwrap());

        handle.draw_line_v(p0.0, p1.0, self.color);
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
