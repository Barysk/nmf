use raylib::prelude::*;

use crate::global::*;

pub struct MainMenu {
    menu_state: MenuState,
    // Idle things
    timer_activity: f32,
    activity_direction_right: bool,
    text_pos_x: f32,
    text_pos_x_mod: f32,
}

enum MenuState {
    Idle, // Default
    Start,
    StartExtra,
    StartPractice,
    Score,
    Option,
    Quit,
}

impl MainMenu {
    const ACTIVITY_TIME_MIN: f32 = 3f32;
    const ACTIVITY_TIME_MAX: f32 = 5f32;
    const TARGET_TEXT_POS: f32 = 100f32;

    pub fn new() -> Self {
        Self {
            menu_state: MenuState::Idle,
            timer_activity: Self::ACTIVITY_TIME_MIN,
            activity_direction_right: false,
            text_pos_x: -256f32,
            text_pos_x_mod: 32f32,
        }
    }

    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        delta_time: &f32,
        cam: &mut Camera3D,
        game_state: &mut GameState,
    ) {
        // exammple update
        rl.update_camera(cam, CameraMode::CAMERA_ORBITAL);

        //self.text_position_x_1 = lerp(self.text_position_x_1, self.text_position_x_2, 0.1f32);

        // Handle text non-stillness
        if self.text_pos_x < Self::TARGET_TEXT_POS {
            self.text_pos_x += 4096f32 * delta_time
        } else {
            if self.timer_activity > 0f32 {
                self.timer_activity -= delta_time;
            } else {
                if self.activity_direction_right {
                    self.text_pos_x_mod += 128f32 * delta_time;
                    if self.text_pos_x_mod >= 32f32 {
                        self.activity_direction_right = false;
                        self.timer_activity =
                            rand::random_range(Self::ACTIVITY_TIME_MIN..Self::ACTIVITY_TIME_MAX);
                    }
                } else if !self.activity_direction_right {
                    self.text_pos_x_mod -= 128f32 * delta_time;
                    if self.text_pos_x_mod <= -32f32 {
                        self.activity_direction_right = true;
                        self.timer_activity =
                            rand::random_range(Self::ACTIVITY_TIME_MIN..Self::ACTIVITY_TIME_MAX);
                    }
                }
            }
        }

        // if compare_floats(&self.text_position_x_1, &self.text_position_x_2, 1f32) {
        //     self.timer_activity -= delta_time;
        // } else {
        //     self.text_position_x_1 = move_from_1_to_2(self.text_position_x_1, &self.text_position_x_2, 100f32, delta_time);
        // }

        // if self.timer_activity <= 0f32{
        //     if self.activity_direction_right == true {
        //         self.activity_direction_right = false;
        //         self.text_position_x_2 = self.text_position_x_2 - 10f32;
        //     } else {
        //         self.activity_direction_right = true;
        //         self.text_position_x_2 = self.text_position_x_2 + 10f32;
        //     }
        // }
    }

    pub fn draw(
        &self,
        thread: &RaylibThread,
        rl: &mut RaylibHandle,
        font: &Font,
        cam: &mut Camera3D,
        render_target: &mut RenderTexture2D,
    ) {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        //d.draw_text("WHOLE WINDOW", 12, 12, 20, Color::ORANGE);
        d.draw_text_ex(
            font,
            "WHOLE WINDOW",
            Vector2::new(12f32, 12f32),
            22f32,
            1f32,
            Color::ORANGE,
        );

        // DRAW IN VIEWPORT
        {
            let mut d = d.begin_texture_mode(&thread, render_target);
            d.clear_background(Color::WHITE);
            d.draw_text_ex(
                font,
                "VIEWPORT",
                Vector2::new(12f32, 12f32),
                22f32,
                1f32,
                Color::ORANGE,
            );

            // DRAW 3D BG
            {
                let mut d = d.begin_mode3D(*cam);
                d.draw_grid(16i32, 1f32);
            }

            // Drawing menu
            {
                d.draw_text_ex(
                    font,
                    "Start",
                    Vector2::new(self.text_pos_x + self.text_pos_x_mod, 640f32 - 64f32 * 6f32),
                    64f32,
                    1f32,
                    Color::BLACK,
                );
                d.draw_text_ex(
                    font,
                    "Start Extra",
                    Vector2::new(self.text_pos_x - self.text_pos_x_mod, 640f32 - 64f32 * 5f32),
                    64f32,
                    1f32,
                    Color::BLACK,
                );
                d.draw_text_ex(
                    font,
                    "Start Practice",
                    Vector2::new(
                        self.text_pos_x + self.text_pos_x_mod * 0.3f32,
                        640f32 - 64f32 * 4f32,
                    ),
                    64f32,
                    1f32,
                    Color::BLACK,
                );
                d.draw_text_ex(
                    font,
                    "Score",
                    Vector2::new(
                        self.text_pos_x - self.text_pos_x_mod * 0.4f32,
                        640f32 - 64f32 * 3f32,
                    ),
                    64f32,
                    1f32,
                    Color::BLACK,
                );
                d.draw_text_ex(
                    font,
                    "Option",
                    Vector2::new(
                        self.text_pos_x + self.text_pos_x_mod * 1.2f32,
                        640f32 - 64f32 * 2f32,
                    ),
                    64f32,
                    1f32,
                    Color::BLACK,
                );
                d.draw_text_ex(
                    font,
                    "Quit",
                    Vector2::new(self.text_pos_x + 20f32, 640f32 - 64f32),
                    64f32,
                    1f32,
                    Color::BLACK,
                );
            }
        }
        draw_on_target(&mut d, &render_target);
    }
}

/// Comparing if two floats are almost equal
fn compare_floats(x: &f32, y: &f32, acceptable_error: f32) -> bool {
    if (x - y).abs() < acceptable_error {
        true
    } else {
        false
    }
}

/// My own interpolation xD
fn move_from_1_to_2(mut from: f32, to: &f32, speed: f32, delta_time: &f32) -> f32 {
    if *to > from {
        from += speed * delta_time;
    } else if *to < from {
        from -= speed * delta_time;
    }
    from
}
