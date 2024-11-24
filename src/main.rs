mod line;
mod action_state;
mod rectangle;
mod pencil;
mod update_execute_action;
mod user_state;

use raylib::prelude::*;
use crate::action_state::ActionState;
use crate::update_execute_action::UpdateExecuteAction;
use crate::user_state::UserState;

const TEXTURE_SIZE: usize = 16;
const TEXTURE_NUMBER: usize = 16;

const BUTTON_SIZE: usize = 32;

const SCREEN_WIDTH: i32 = 1020;
const SCREEN_HEIGHT: i32 = 510;

const CANVAS_MARGIN: f32 = 4f32;

const TRANSPARENT: Color = Color::new(0,0,0,0);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("PaintIt")
        .build();

    let textures = rl.load_texture(&thread, "resources/tools.png").unwrap();
    let mut atlas_sources: [Rectangle; TEXTURE_NUMBER] = Default::default();
    for i in 0..TEXTURE_NUMBER {
        atlas_sources[i] = Rectangle { x: (i * TEXTURE_SIZE) as f32, y: 0f32, width: TEXTURE_SIZE as f32, height: TEXTURE_SIZE as f32 };
    }

    let mut button_positions: [Vector2; TEXTURE_NUMBER] = Default::default();
    for i in 0..TEXTURE_NUMBER {
        button_positions[i] = Vector2 { x: ((i % 2) * BUTTON_SIZE) as f32, y: ((i / 2) * BUTTON_SIZE) as f32 };
    }

    let mut button_bounds: [Rectangle; TEXTURE_NUMBER] = Default::default();
    for i in 0..TEXTURE_NUMBER {
        button_bounds[i] = Rectangle {
            x: button_positions[i].x,
            y: button_positions[i].y,
            width: (TEXTURE_SIZE + 16) as f32,
            height: (TEXTURE_SIZE + 16) as f32
        };
    }

    let mut current_pressed: Option<ActionState> = None;
    let mut current_colors: [Color; 2] = [Color::BLACK, Color::WHITE];

    let mut canvas_position = Vector2 { x: (BUTTON_SIZE * 2) as f32 + CANVAS_MARGIN, y: CANVAS_MARGIN };
    let mut canvas_image = Image::gen_image_color(743, 406, TRANSPARENT);

    let mut canvas_dirty = false;
    let mut canvas_texture =  rl.load_texture_from_image(&thread, &canvas_image).unwrap();

    while !rl.window_should_close() {
        // Update
        let mouse_position = rl.get_mouse_position();
        let user_state = UserState {
            mouse_position,
            current_colors,
            canvas_position,
        };

        for i in 0..TEXTURE_NUMBER {
            let button_pressed = button_bounds[i].check_collision_point_rec(mouse_position)
                && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

            if button_pressed {
                let button = ActionState::try_from(i as u32).unwrap();
                println!("pressed {:?}", button);
                current_pressed = Option::from(button);
            }
        }

        let canvas_rectangle = Rectangle { x: canvas_position.x, y: canvas_position.y, width: canvas_image.width as f32, height: canvas_image.height as f32 };
        let mouse_in_canvas = canvas_rectangle.check_collision_point_rec(mouse_position);
        let canvas_pressed = mouse_in_canvas
            && rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

        match current_pressed {
            Some(ref mut generic_state) => {
                specify_state!(generic_state, specific_state, {
                    if canvas_pressed {
                        specific_state.update_pressed(user_state);
                    } else {
                        specific_state.update_unpressed(user_state);
                    }

                    canvas_dirty = specific_state.draw(&mut canvas_image);
                    if canvas_dirty {
                        println!("{:?}", specific_state);
                        specific_state.update_after_draw(user_state);
                    }
                });
            }
            _ => {}
        }


        if canvas_dirty {
            canvas_texture = rl.load_texture_from_image(&thread, &canvas_image).unwrap();
            canvas_dirty = false;
        }

        // Draw
        let mut handle = rl.begin_drawing(&thread);

        handle.clear_background(Color::GRAY);

        handle.draw_rectangle(0, 0, (BUTTON_SIZE * 2) as i32, SCREEN_HEIGHT-20, Color::LIGHTGRAY);
        handle.draw_rectangle_lines(0, 0, (BUTTON_SIZE * 2) as i32, SCREEN_HEIGHT-20, Color::BLACK);

        for i in 0..TEXTURE_NUMBER {
            let position = Vector2 { x: button_positions[i].x + 8f32, y: button_positions[i].y + 8f32 };

            if current_pressed.is_some_and(|b|  <ActionState as Into<u32>>::into(b) == (i as u32)) {
                // Draw the pressed button
                handle.draw_texture_rec(&textures, atlas_sources[i], position, Color::WHITE);
            } else {
                // Draw the button
                handle.draw_texture_rec(&textures, atlas_sources[i], position, Color::WHITE);
                handle.draw_rectangle_lines_ex(button_bounds[i], 1f32, Color::BLACK);
                handle.draw_line(
                    (button_bounds[i].x + 1f32) as i32,
                    button_bounds[i].y as i32,
                    (button_bounds[i].x + button_bounds[i].width - 1f32) as i32,
                    button_bounds[i].y as i32,
                    Color::WHITE
                );
                handle.draw_line(
                    (button_bounds[i].x + 1f32) as i32,
                    button_bounds[i].y as i32,
                    button_bounds[i].x as i32,
                    (button_bounds[i].y + button_bounds[i].height - 1f32) as i32,
                    Color::WHITE
                );
            }
        }

        handle.draw_rectangle(0, SCREEN_HEIGHT-20, SCREEN_WIDTH, 20, Color::LIGHTGRAY);
        handle.draw_line(0, SCREEN_HEIGHT-20, SCREEN_WIDTH, SCREEN_HEIGHT-20, Color::WHITE);

        handle.draw_rectangle_rec(canvas_rectangle, Color::WHITE);
        handle.draw_texture_v(&canvas_texture, canvas_position, Color::WHITE); // TODO fix tint

        match current_pressed {
            Some(ref mut generic_state) => {
                specify_state!(generic_state, specific_state, {
                    specific_state.draw_state(&mut handle, user_state);
                });
            }
            _ => {}
        }

        if mouse_in_canvas {
            let text = format!("{},{}", mouse_position.x, mouse_position.y);
            handle.draw_text(&text, SCREEN_WIDTH - 150, SCREEN_HEIGHT - 15,
                             12, Color::BLACK);
        }
    }
}