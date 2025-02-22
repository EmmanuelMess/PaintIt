use std::rc::Rc;
use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::{CanvasVector2, UserState};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle};
use raylib::prelude::{Image};
use raylib::{RaylibHandle, RaylibThread};
use raylib::texture::Texture2D;
use crate::raylib_extensions;

const TRANSPARENT: Color = Color::new(0, 0, 0, 0);

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum PasteMode {
    #[default]
    Opaque,
    Transparent,
}

#[derive(Debug, Clone, Default)]
enum MoveStateMachine {
    #[default]
    Nothing,
    StartSelected { start: CanvasVector2, end: CanvasVector2 },
    AreaSelected { start: CanvasVector2, end: CanvasVector2, selected_image: Image,
        selected_texture: Rc<Texture2D>  },
    Moving { start: CanvasVector2, end: CanvasVector2, last_mouse_position: CanvasVector2,
        selected_image: Image, selected_texture: Rc<Texture2D> },
    Draw { start: CanvasVector2, end: CanvasVector2, selected_image: Image,
        selected_texture: Rc<Texture2D> },
}

/// Delete the selected area
#[derive(Debug, Default, Clone)]
struct DeleteSection {
    start: CanvasVector2,
    end: CanvasVector2,
}

#[derive(Debug, Default, Clone)]
pub struct SelectState {
    state: MoveStateMachine,
    delete_section: Option<DeleteSection>
}

impl UpdateExecuteAction for SelectState {
    fn update_pressed(&mut self, user_state: &UserState, _: &mut RaylibHandle, _: &RaylibThread) {
        let mouse_position = user_state.to_canvas(user_state.mouse_position);

        self.state = match self.clone().state {
            MoveStateMachine::Nothing => {
                MoveStateMachine::StartSelected {
                    start: mouse_position,
                    end: mouse_position,
                }
            },
            MoveStateMachine::StartSelected { start, end: _ } => {
                MoveStateMachine::StartSelected {
                    start,
                    end: mouse_position,
                }
            },
            MoveStateMachine::AreaSelected { start , end, selected_image,
                selected_texture} => {
                let rectangle = raylib_extensions::generate_rectangle(start.0, end.0);

                if rectangle.check_collision_point_rec(mouse_position.0) {
                    MoveStateMachine::Moving {
                        start, end, last_mouse_position: mouse_position, selected_image, selected_texture
                    }
                } else {
                    MoveStateMachine::Draw { start, end, selected_image, selected_texture }
                }
            },
            MoveStateMachine::Moving { start, end,
                last_mouse_position, selected_image, selected_texture } => {
                let mouse_delta = mouse_position.0 - last_mouse_position.0;
                let new_start = CanvasVector2(start.0 + mouse_delta);
                let new_end = CanvasVector2(end.0 + mouse_delta);
                MoveStateMachine::Moving {
                    start: new_start, end: new_end, last_mouse_position: mouse_position,
                    selected_image, selected_texture
                }
            },
            MoveStateMachine::Draw { start, end,
                selected_image, selected_texture } => {
                // Only get out of Draw if it has actually been drawn
                MoveStateMachine::Draw { start, end, selected_image, selected_texture }
            },
        };
    }

    fn update_unpressed(&mut self, user_state: &UserState, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.state = match self.clone().state {
            MoveStateMachine::Nothing => MoveStateMachine::Nothing,
            MoveStateMachine::StartSelected { start, end } => {
                if start == end {
                    // Too small, also prevents other issues
                    MoveStateMachine::Nothing
                } else {
                    let rectangle_src = raylib_extensions::generate_rectangle(start.0, end.0);
                    let rectangle_dst = Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: rectangle_src.width,
                        height: rectangle_src.height
                    };
                    let mut selected_image = Image::gen_image_color(rectangle_src.width as i32,
                                                                    rectangle_src.width as i32,
                                                                    TRANSPARENT);
                    selected_image.draw(&user_state.canvas_image, rectangle_src, rectangle_dst,
                                        Color::BLACK);
                    let texture = rl.load_texture_from_image(&thread, &selected_image)
                        .unwrap();
                    let selected_texture = Rc::new(texture);

                    self.delete_section = Option::from(DeleteSection { start, end });

                    MoveStateMachine::AreaSelected { start, end, selected_image, selected_texture }
                }
            },
            MoveStateMachine::AreaSelected { start, end,
                selected_image, selected_texture }
            | MoveStateMachine::Moving { start, end, last_mouse_position: _,
            selected_image, selected_texture } => {
                MoveStateMachine::AreaSelected { start, end, selected_image, selected_texture }
            },
            MoveStateMachine::Draw { start, end,
                selected_image, selected_texture } => {
                // Only get out of Draw if it has actually been drawn
                MoveStateMachine::Draw { start, end, selected_image, selected_texture }
            },
        };
    }

    fn update_after_draw(&mut self, _: &UserState) {}

    fn draw(&mut self, image: &mut Image) -> bool {
        if let Some(delete_section) = self.clone().delete_section {
            let rectangle_dst =
                raylib_extensions::generate_rectangle(delete_section.start.0, delete_section.end.0);
            image.draw_rectangle(rectangle_dst.x as i32, rectangle_dst.y as i32,
                                 rectangle_dst.width as i32, rectangle_dst.height as i32,
                                 Color::WHITE);
            self.delete_section = None;
            return true
        }

        match self.state.clone() {
            MoveStateMachine::Nothing
            | MoveStateMachine::StartSelected { .. }
            | MoveStateMachine::AreaSelected { .. }
            | MoveStateMachine::Moving { .. } => {
                false
            },
            MoveStateMachine::Draw { start, end,
                selected_image, selected_texture: _selected_texture
            } => {
                let rectangle_dst = raylib_extensions::generate_rectangle(start.0, end.0);
                let rectangle_src = Rectangle { x: 0.0, y: 0.0, width: rectangle_dst.width,
                    height: rectangle_dst.height };
                image.draw(&selected_image, rectangle_src, rectangle_dst, Color::WHITE);
                self.state = MoveStateMachine::Nothing;
                true
            },
        }
    }

    fn draw_state(&self, user_state: &UserState, handle: &mut RaylibDrawHandle, _: &RaylibThread) {
        match self.state.clone() {
            MoveStateMachine::Nothing => {},
            MoveStateMachine::StartSelected { start, end } => {
                let p0 = user_state.to_window(start).0;
                let p1 = user_state.to_window(end).0;
                let rectangle = raylib_extensions::generate_rectangle(p0, p1);
                handle.draw_rectangle_lines_ex(rectangle, 1.0, Color::BLACK);
            },
            | MoveStateMachine::AreaSelected { start, end,
                selected_image: _, selected_texture }
            | MoveStateMachine::Moving { start, end,
                last_mouse_position: _, selected_image: _, selected_texture }
            | MoveStateMachine::Draw { start, end,
                selected_image: _, selected_texture } => {
                let p0 = user_state.to_window(start).0;
                let p1 = user_state.to_window(end).0;
                let rectangle = raylib_extensions::generate_rectangle(p0, p1);
                handle.draw_rectangle_lines_ex(rectangle, 1.0, Color::BLACK);
                handle.draw_texture_ex(&*selected_texture, p0, 0.0, 1.0, Color::WHITE); // TODO fix tint
            },
        }
    }

    fn get_color(&self) -> Option<Color> { None }
}
