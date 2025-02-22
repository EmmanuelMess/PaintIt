use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct BucketState {
    mouse_position: Option<CanvasVector2>,
    color: Color,
}

impl UpdateExecuteAction for BucketState {
    fn update_pressed(&mut self, user_state: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        self.mouse_position = Option::from(user_state.to_canvas(user_state.mouse_position));
        self.color = user_state.current_colors[0];
    }

    fn update_unpressed(&mut self, _: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        self.mouse_position = None;
    }

    fn update_after_draw(&mut self, _: &UserState) {}

    fn draw(&mut self, image: &mut Image) -> bool {
        match self.mouse_position {
            Some(mouse_position) => {
                // TODO make cleaner and faster

                let replaced_color =
                    image.get_color(
                        mouse_position.0.x as i32,
                        mouse_position.0.y as i32
                    );

                let mut repaint = HashSet::<(i32, i32)>::new();
                let mut stack = Vec::<Vector2>::new();
                stack.push(mouse_position.0);
                while let Some(node) = stack.pop() {
                    if repaint.contains(&(node.x as i32, node.y as i32)) {
                        continue;
                    }

                    if image.get_color(node.x as i32, node.y as i32) != replaced_color {
                        continue;
                    }

                    repaint.insert((node.x as i32, node.y as i32));

                    if 0f32 <= node.x - 1f32 {
                        stack.push(node + Vector2 { x: -1f32, y:  0f32 } );
                    }

                    if 0f32 <= node.y - 1f32 {
                        stack.push(node + Vector2 { x:  0f32, y: -1f32 } );
                    }

                    if node.x + 1f32 < image.width as f32 {
                        stack.push(node + Vector2 { x: 1f32, y:  0f32 } );
                    }

                    if node.y + 1f32 < image.height as f32 {
                        stack.push(node + Vector2 { x:  0f32, y: 1f32 } );
                    }
                }

                for (x, y) in repaint {
                    image.draw_pixel( x, y, self.color);
                }

                true
            }
            _ => false,
        }
    }

    fn draw_state(&self, _: &UserState, _: &mut RaylibDrawHandle, _: &RaylibThread) {}

    fn get_color(&self) -> Option<Color> {
        None
    }
}
