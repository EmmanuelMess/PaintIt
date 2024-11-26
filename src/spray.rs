use std::f32::consts::TAU;
use raylib::color::Color;
use raylib::consts::PI;
use raylib::drawing::RaylibDrawHandle;
use raylib::math;
use raylib::math::Vector2;
use raylib::prelude::Image;
use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::UserState;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SpraySize {
    SizeOne,
    SizeTwo,
    SizeThree,
}

impl SpraySize {
    fn radius(self) -> f32 {
        match self {
            SpraySize::SizeOne => 5f32,
            SpraySize::SizeTwo => 10f32,
            SpraySize::SizeThree => 20f32,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct SprayState {
    mouse_position_in_canvas: Option<Vector2>,
    radius: f32,
    color: Color,
}

impl UpdateExecuteAction for SprayState {
    fn update_pressed(&mut self, user_state: UserState) {
        self.mouse_position_in_canvas = Option::from(user_state.to_canvas(user_state.mouse_position));
        self.radius = user_state.spray_size.radius();
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, _: UserState) {
        self.mouse_position_in_canvas = None;
    }

    fn update_after_draw(&mut self, _: UserState) {}

    fn draw(&self, image: &mut Image) -> bool {
        match self.mouse_position_in_canvas {
            Some(mouse_position_in_canvas) => {
                let theta = rand::random::<f32>() * TAU;
                let radius = rand::random::<f32>() * self.radius;
                let position = Vector2 {
                    x: radius * f32::cos(theta),
                    y: radius * f32::sin(theta),
                };

                image.draw_pixel_v(mouse_position_in_canvas + position, self.color);
                true
            }
            None => false,
        }
    }

    fn draw_state(&self, _: &mut RaylibDrawHandle, _: UserState) {}
}
