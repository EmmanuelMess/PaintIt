use raylib::prelude::*;
use crate::actions::action_state::ActionState;
use crate::actions::brush::{BrushSize, BrushType};
use crate::specify_state;
use crate::actions::spray::SpraySize;
use crate::actions::update_execute_action::UpdateExecuteAction;
use crate::user_state::{WindowVector2, UserState};

const TEXTURE_SIZE: usize = 16;
const TEXTURE_NUMBER: usize = 16;

const BUTTON_SIZE: usize = 32;

const SCREEN_WIDTH: i32 = 1020;
const SCREEN_HEIGHT: i32 = 510;

const CANVAS_MARGIN: f32 = 4f32;

const TRANSPARENT: Color = Color::new(0,0,0,0);

pub struct Ui {
    rl: RaylibHandle,
    thread: RaylibThread,
    textures: Texture2D,
    atlas_sources: [Rectangle; TEXTURE_NUMBER],
    button_positions: [Vector2; TEXTURE_NUMBER],
    button_bounds: [Rectangle; TEXTURE_NUMBER],
    current_pressed: Option<ActionState>,
    current_colors: [Color; 2],
    canvas_position: Vector2,
    canvas_image: Image,
    canvas_dirty: bool,
    canvas_texture: Texture2D,
    canvas_rectangle: Rectangle,
    mouse_position: WindowVector2,
    mouse_in_canvas: bool,
    /// This user state is to be passed to the actions
    user_state: UserState,
}

impl Ui {
    pub fn init() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("PaintIt")
            .build();

        let textures = rl.load_texture(&thread, "resources/tools.png").unwrap();
        let mut atlas_sources: [Rectangle; TEXTURE_NUMBER] = Default::default();
        for i in 0..TEXTURE_NUMBER {
            atlas_sources[i] = Rectangle {
                x: (i * TEXTURE_SIZE) as f32,
                y: 0f32,
                width: TEXTURE_SIZE as f32,
                height: TEXTURE_SIZE as f32
            };
        }

        let mut button_positions: [Vector2; TEXTURE_NUMBER] = Default::default();
        for i in 0..TEXTURE_NUMBER {
            button_positions[i] = Vector2 {
                x: ((i % 2) * BUTTON_SIZE) as f32,
                y: ((i / 2) * BUTTON_SIZE) as f32
            };
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

        let canvas_position = Vector2 {
            x: (BUTTON_SIZE * 2) as f32 + CANVAS_MARGIN,
            y: CANVAS_MARGIN
        };
        let mut canvas_image = Image::gen_image_color(743, 406, TRANSPARENT);

        let mut canvas_dirty = false;
        let mut canvas_texture = rl.load_texture_from_image(&thread, &canvas_image).unwrap();

        let canvas_rectangle = Rectangle {
            x: canvas_position.x,
            y: canvas_position.y,
            width: canvas_image.width as f32,
            height: canvas_image.height as f32
        };

        let mouse_position = WindowVector2(Vector2::new(0.0, 0.0));
        let user_state = UserState {
            mouse_position,
            current_colors: current_colors,
            canvas_position: WindowVector2(canvas_position),
            spray_size: SpraySize::SizeOne,
            brush_size: BrushSize::Two,
            brush_type: BrushType::Circle,
            canvas_image: canvas_image.clone(),
        };

        return Ui {
            rl,
            thread,
            textures,
            atlas_sources,
            button_positions,
            button_bounds,
            current_pressed,
            current_colors,
            canvas_position,
            canvas_image,
            canvas_dirty,
            canvas_texture,
            canvas_rectangle,
            mouse_position,
            mouse_in_canvas: false,
            user_state,
        };
    }

    fn update(&mut self) {
        self.mouse_position = WindowVector2(self.rl.get_mouse_position());
        self.user_state = UserState {
            mouse_position: self.mouse_position,
            current_colors: self.current_colors,
            canvas_position: WindowVector2(self.canvas_position),
            spray_size: SpraySize::SizeOne,
            brush_size: BrushSize::Two,
            brush_type: BrushType::Circle,
            canvas_image: self.canvas_image.clone(),
        };

        for i in 0..TEXTURE_NUMBER {
            let button_pressed = self.button_bounds[i].check_collision_point_rec(self.mouse_position.0)
                && self.rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);

            if button_pressed {
                let button = ActionState::try_from(i as u32).unwrap();
                println!("pressed {:?}", button);
                self.current_pressed = Option::from(button);
            }
        }

        let mouse_in_canvas = self.canvas_rectangle.check_collision_point_rec(self.mouse_position.0);
        let canvas_pressed = mouse_in_canvas
            && self.rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

        //TODO change to previous tool after color picker
        if let Some(generic_state) = self.current_pressed.as_deref_mut() {
            specify_state!(generic_state, specific_state, {
                if canvas_pressed {
                    specific_state.update_pressed(&self.user_state, &mut self.rl, &self.thread);
                } else {
                    specific_state.update_unpressed(&self.user_state, &mut self.rl, &self.thread);
                }

                // TODO use a layer system to allow for do-undo
                self.canvas_dirty = specific_state.draw(&mut self.canvas_image);

                if self.canvas_dirty {
                    println!("{:?}", specific_state);
                    specific_state.update_after_draw(&self.user_state);
                }
                if let Some(color) =  specific_state.get_color() {
                    self.current_colors[0] = color;
                }
             });
        }

        if self.canvas_dirty {
            self.canvas_texture = self.rl.load_texture_from_image(&self.thread, &self.canvas_image).unwrap();
            self.canvas_dirty = false;
        }
    }

    fn draw(&mut self) {
        let mut handle = self.rl.begin_drawing(&self.thread);

        handle.clear_background(Color::GRAY);

        handle.draw_rectangle(0, 0,
                              (BUTTON_SIZE * 2) as i32, SCREEN_HEIGHT - 20,
                              Color::LIGHTGRAY);
        handle.draw_rectangle_lines(0, 0,
                                    (BUTTON_SIZE * 2) as i32, SCREEN_HEIGHT - 20,
                                    Color::BLACK);

        for i in 0..TEXTURE_NUMBER {
            let position = Vector2 {
                x: self.button_positions[i].x + 8f32,
                y: self.button_positions[i].y + 8f32
            };

            if self.current_pressed.as_ref().is_some_and(|b| u32::from(b) == (i as u32)) {
                // Draw the pressed button
                handle.draw_texture_rec(&self.textures, self.atlas_sources[i], position, Color::WHITE);
            } else {
                // Draw the button
                handle.draw_texture_rec(&self.textures, self.atlas_sources[i], position, Color::WHITE);
                handle.draw_rectangle_lines_ex(self.button_bounds[i], 1f32, Color::BLACK);
                handle.draw_line(
                    (self.button_bounds[i].x + 1f32) as i32,
                    self.button_bounds[i].y as i32,
                    (self.button_bounds[i].x + self.button_bounds[i].width - 1f32) as i32,
                    self.button_bounds[i].y as i32,
                    Color::WHITE
                );
                handle.draw_line(
                    (self.button_bounds[i].x + 1f32) as i32,
                    self.button_bounds[i].y as i32,
                    self.button_bounds[i].x as i32,
                    (self.button_bounds[i].y + self.button_bounds[i].height - 1f32) as i32,
                    Color::WHITE
                );
            }
        }

        handle.draw_rectangle(0, SCREEN_HEIGHT - 20, SCREEN_WIDTH, 20, Color::LIGHTGRAY);
        handle.draw_line(0, SCREEN_HEIGHT - 20, SCREEN_WIDTH, SCREEN_HEIGHT - 20, Color::WHITE);

        handle.draw_rectangle_rec(self.canvas_rectangle, Color::WHITE);
        handle.draw_texture_v(&self.canvas_texture, self.canvas_position, Color::WHITE); // TODO fix tint

        if let Some(generic_state) = self.current_pressed.as_deref_mut() {
            specify_state!(generic_state, specific_state, {
                    specific_state.draw_state(&self.user_state, &mut handle, &self.thread);
                });
        }

        if self.mouse_in_canvas {
            let text = format!("{},{}", self.mouse_position.0.x, self.mouse_position.0.y);
            handle.draw_text(&text, SCREEN_WIDTH - 150, SCREEN_HEIGHT - 15,
                             12, Color::BLACK);
        }
    }

    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            self.update();
            self.draw();
        }
    }
}