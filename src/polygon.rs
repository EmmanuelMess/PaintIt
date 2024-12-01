use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PolygonState {
    points: Vec<CanvasVector2>,
    // Invariant: new_point != None => points.len() >= 1
    new_point: Option<CanvasVector2>,
    color: Color,
}

const POLYGON_CLOSE_DISTANCE: f32 = 5.0;

impl UpdateExecuteAction for PolygonState {
    fn update_pressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // First point gets added immediately, after the first, add them on press
        if self.points.is_empty() {
            self.points.push(user_state.to_canvas(user_state.mouse_position));
        } else {
            self.new_point = Some(user_state.to_canvas(user_state.mouse_position));
        }
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread) {
        if let Some(new_point) = self.new_point {
            let first = self.points.first().unwrap();

            if self.points.len() == 1 {
                // Don't start polygons that would close instantly
                if first.0.distance_to(new_point.0) > POLYGON_CLOSE_DISTANCE {
                    self.points.push(new_point);
                } else {
                    self.new_point = None;
                    self.points.clear();
                }
            } else {
                self.points.push(new_point);
                self.new_point = None;
            }
        }
    }

    fn update_after_draw(&mut self, user_state: &UserState) {
    }

    fn draw(&mut self, image: &mut Image) -> bool {
        if self.points.len() < 2 {
            return false;
        }

        {
            let first = self.points.first().unwrap();
            let last = self.points.last().unwrap();

            if first.0.distance_to(last.0) > POLYGON_CLOSE_DISTANCE {
                return false;
            }
        }

        {
            // Ignore the last point (intended behaviour)
            let actual_points = &self.points[0..self.points.len()-1];
            let actual_last = self.points[actual_points.len() - 1].0;
            let actual_first = self.points.first().unwrap().0;

            for i in 1..actual_points.len() {
                let p0 = actual_points[i - 1].0;
                let p1 = actual_points[i].0;

                image.draw_line_v(p0, p1, self.color);
            }

            // Ignore the last point (intended behaviour)
            image.draw_line_v(actual_last, actual_first, self.color);

            self.new_point = None;
            self.points.clear();
            true
        }
    }

    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, thread: &RaylibThread) {
        for i in 1..self.points.len() {
            let p0 = user_state.to_window(self.points[i-1]).0;
            let p1 = user_state.to_window(self.points[i]).0;

            handle.draw_line_v(p0, p1, self.color);
        }

        if let Some(new_point) = self.new_point {
            let p0 = user_state.to_window(self.points[self.points.len() - 1]).0;
            let p1 = user_state.to_window(new_point).0;

            handle.draw_line_v(p0, p1, self.color);
        }
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
