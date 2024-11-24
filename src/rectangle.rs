use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::Image;
use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::UserState;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct RectangleState {
    start: Option<Vector2>,
    end: Option<Vector2>,
    draw_now: bool,
    color: Color,
}

impl UpdateExecuteAction for RectangleState {
    fn update_pressed(&mut self, user_state: UserState) {
        print!("{:?}, {:?}",  self.start,  self.end);
        if self.start == None {
            self.start = Option::from(user_state.to_canvas(user_state.mouse_position));
        } else {
            self.end = Option::from(user_state.to_canvas(user_state.mouse_position));
        }
        println!(" {:?}, {:?}",  self.start,  self.end);

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

    fn draw(&self, image: &mut Image) -> bool {
        if self.draw_now {
            let p0 = self.start.unwrap();
            let p1 = self.end.unwrap();

            let start = if p0.x <= p1.x && p0.y <= p1.y { p0 } else { p1 };
            let end = if p0.x <= p1.x && p0.y <= p1.y { p1 } else { p0 };

            let size = end - start;

            let rectangle = Rectangle {
                x: start.x,
                y: start.y,
                width: size.x.abs(),
                height: size.y.abs(),
            };
            image.draw_rectangle_lines(rectangle, 1, self.color);
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

        let start = if p0.x <= p1.x && p0.y <= p1.y { p0 } else { p1 };
        let end = if p0.x <= p1.x && p0.y <= p1.y { p1 } else { p0 };

        let size = end - start;

        let rectangle = Rectangle {
            x: start.x,
            y: start.y,
            width: size.x.abs(),
            height: size.y.abs(),
        };
        handle.draw_rectangle_lines_ex(rectangle, 1f32, self.color);
    }
}
