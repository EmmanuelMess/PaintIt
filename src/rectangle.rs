use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Rectangle;
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct RectangleState {
    start: Option<CanvasVector2>,
    end: Option<CanvasVector2>,
    draw_now: bool,
    color: Color,
}

impl UpdateExecuteAction for RectangleState {
    fn update_pressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if self.start == None {
            self.start = Option::from(user_state.to_canvas(user_state.mouse_position));
        } else {
            self.end = Option::from(user_state.to_canvas(user_state.mouse_position));
        }

        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if self.start != None && self.end != None {
            self.draw_now = true;
        }
    }

    fn update_after_draw(&mut self, user_state: &UserState) {
        if self.draw_now {
            self.start = None;
            self.end = None;
            self.draw_now = false;
        }
    }

    fn draw(&mut self, image: &mut Image) -> bool {
        if !(self.draw_now) {
            return false;
        }

        let p0 = self.start.unwrap();
        let p1 = self.end.unwrap();

        let size = p0.0 - p1.0;

        let rectangle = Rectangle {
            x: f32::min(p0.0.x, p1.0.x),
            y: f32::min(p0.0.y, p1.0.y),
            width: size.x.abs(),
            height: size.y.abs(),
        };
        image.draw_rectangle_lines(rectangle, 1, self.color);
        true
    }

    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, thread: &RaylibThread) {
        if !(self.start != None && self.end != None) {
            return;
        }

        let p0 = user_state.to_window(self.start.unwrap());
        let p1 = user_state.to_window(self.end.unwrap());

        let size = p0.0 - p1.0;

        let rectangle = Rectangle {
            x: f32::min(p0.0.x, p1.0.x),
            y: f32::min(p0.0.y, p1.0.y),
            width: size.x.abs(),
            height: size.y.abs(),
        };
        handle.draw_rectangle_lines_ex(rectangle, 1f32, self.color);
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
