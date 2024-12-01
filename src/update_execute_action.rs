use raylib::color::Color;
use crate::user_state::UserState;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::Image;
use raylib::{RaylibHandle, RaylibThread};

pub trait UpdateExecuteAction {
    /// Called when the pointer inside the canvas, and the left mouse button is pressed
    fn update_pressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread);
    /// Called when the pointer inside the canvas, and the left mouse button is not pressed
    fn update_unpressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn update_after_draw(&mut self, user_state: &UserState);

    /// Draw onto the canvas
    fn draw(&mut self, image: &mut Image) -> bool;
    /// Draw onto the temporary layer over the canvas, this layer will be cleared after each frame
    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, thread: &RaylibThread);

    fn get_color(&self) -> Option<Color>;
}
