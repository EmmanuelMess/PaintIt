use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};
use std::f32::consts::TAU;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct EllipseState {
    start: Option<CanvasVector2>,
    end: Option<CanvasVector2>,
    draw_now: bool,
    color: Color,
}

impl UpdateExecuteAction for EllipseState {
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
        if !self.draw_now {
            return false;
        }

        let p0 = self.start.unwrap().0;
        let p1 = self.end.unwrap().0;

        let middle = (p0 + p1) / 2.0;

        let a = (p0.x - p1.x).abs() / 2.0;
        let b = (p0.y - p1.y).abs() / 2.0;

        let mut t = 0f32;
        while t < TAU {
            let px = middle.x + a * f32::cos(t);
            let py = middle.y + b * f32::sin(t);

            image.draw_pixel(px as i32, py as i32, self.color);

            t += 0.001;
        }
        true
    }

    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, thread: &RaylibThread) {
        if let Some(start) = self.start {
            if let Some(end) = self.end {
                let p0 = user_state.to_window(start).0;
                let p1 = user_state.to_window(end).0;

                let middle = (p0 + p1) / 2.0;

                let a = (p0.x - p1.x).abs() / 2.0;
                let b = (p0.y - p1.y).abs() / 2.0;

                let mut t = 0f32;
                while t < TAU {
                    let px = middle.x + a * f32::cos(t);
                    let py = middle.y + b * f32::sin(t);

                    handle.draw_pixel(px as i32, py as i32, self.color);

                    t += 0.001;
                }
            }
        }
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
