use crate::actions::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};
use std::f32::consts::TAU;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct RoundedRectangleState {
    start: Option<CanvasVector2>,
    end: Option<CanvasVector2>,
    draw_now: bool,
    color: Color,
}

macro_rules! draw_rounded_rectangle {
    ($base: ident, $p0: ident, $p1: ident, $color: ident) => {
        let size = $p0 - $p1;

        let r = Rectangle {
            x: f32::min($p0.x, $p1.x),
            y: f32::min($p0.y, $p1.y),
            width: size.x.abs(),
            height: size.y.abs(),
        };

        let segments = 10;
        let line_thick = 1.0;
        let step_length = (TAU / 4.0)/(segments as f32);
        let radius = f32::min(10.0, f32::min(r.height/2.0, r.width/2.0));
        let outer_radius = radius + line_thick;
        let inner_radius = radius;
        let angles: [f32; 4] = [TAU / 2.0, TAU / 4.0 * 3.0, 0.0, TAU / 4.0];
        let point: [Vector2; 16] = [
            Vector2::new(r.x + inner_radius, r.y - line_thick),
            Vector2::new((r.x + r.width) - inner_radius, r.y - line_thick),
            Vector2::new( r.x + r.width + line_thick, r.y + inner_radius ),
            Vector2::new(r.x + r.width + line_thick, (r.y + r.height) - inner_radius),
            Vector2::new((r.x + r.width) - inner_radius, r.y + r.height + line_thick),
            Vector2::new(r.x + inner_radius, r.y + r.height + line_thick),
            Vector2::new( r.x - line_thick, (r.y + r.height) - inner_radius),
            Vector2::new(r.x - line_thick, r.y + inner_radius),
            Vector2::new(r.x + inner_radius, r.y),
            Vector2::new((r.x + r.width) - inner_radius, r.y),
            Vector2::new( r.x + r.width, r.y + inner_radius ),
            Vector2::new(r.x + r.width, (r.y + r.height) - inner_radius),
            Vector2::new((r.x + r.width) - inner_radius, r.y + r.height),
            Vector2::new(r.x + inner_radius, r.y + r.height),
            Vector2::new(r.x, (r.y + r.height) - inner_radius),
            Vector2::new(r.x, r.y + inner_radius),
        ];
        let centers: [Vector2; 4] = [
            Vector2::new(r.x + inner_radius, r.y + inner_radius),
            Vector2::new((r.x + r.width) - inner_radius, r.y + inner_radius),
            Vector2::new((r.x + r.width) - inner_radius, (r.y + r.height) - inner_radius),
            Vector2::new(r.x + inner_radius, (r.y + r.height) - inner_radius),
        ];

        for k in 0..4 {
            let mut angle = angles[k];
            let center = centers[k];

            for _ in 0..segments {
                $base.draw_line_v(
                    Vector2::new(
                        center.x + f32::cos(angle)*outer_radius,
                        center.y + f32::sin(angle)*outer_radius
                    ),
                    Vector2::new(
                        center.x + f32::cos(angle + step_length)*outer_radius,
                        center.y + f32::sin(angle + step_length)*outer_radius
                    ),
                    $color,
                );
                angle += step_length;
            }
        }

        // And now the remaining 4 lines
        for i in (0..8).step_by(2) {
            $base.draw_line_v(
                Vector2::new(point[i].x, point[i].y),
                Vector2::new(point[i + 1].x, point[i + 1].y),
                $color,
            );
        }
    };
}

impl UpdateExecuteAction for RoundedRectangleState {
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
        if !(self.draw_now) {
            return false;
        }

        if let Some(start) = self.start {
            if let Some(end) = self.end {
                let p0 = start.0;
                let p1 = end.0;
                let color = self.color;
                draw_rounded_rectangle!(image, p0, p1, color);
            }
        }
        true
    }

    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, _: &RaylibThread) {
        if let Some(start) = self.start {
            if let Some(end) = self.end {
                let p0 = user_state.to_window(start).0;
                let p1 = user_state.to_window(end).0;
                let color = self.color;
                draw_rounded_rectangle!(handle, p0, p1, color);
            }
        }
    }

    fn get_color(&self) -> Option<Color> {
        None
    }
}
