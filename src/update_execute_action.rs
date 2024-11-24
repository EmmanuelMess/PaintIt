use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::prelude::Image;
use crate::user_state::UserState;

pub trait UpdateExecuteAction {
    fn update_pressed(&mut self, user_state: UserState);
    fn update_unpressed(&mut self, user_state: UserState);
    fn update_after_draw(&mut self, user_state: UserState);

    fn draw(&self, image: &mut Image) -> bool;
    fn draw_state(&self, handle: &mut RaylibDrawHandle, user_state: UserState);
}
