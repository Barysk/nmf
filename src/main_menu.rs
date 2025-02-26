use raylib::prelude::*;

use crate::global::*;

pub struct MainMenu {
    menu_state: MenuState,
    // Main menu things
    main_menu_activity: MenuActivity,
    activity_direction_right: bool,
    text_pos_x: f32,
    text_pos_x_mod: f32,
    timer_activity: f32,
    chosen_index: u8,
    // Option things
    option_activity: MenuActivity,
}

enum MenuActivity {
    Show,
    Idle,
    Hide,
}

enum MenuState {
    Idle,          // Default
    Start,         // 2
    StartExtra,    // 6
    StartPractice, // 3
    Score,         // 5
    Option,        // 1
    Quit,          // Quit
}

impl MainMenu {
    const ACTIVITY_TIME_MIN: f32 = 3f32;
    const ACTIVITY_TIME_MAX: f32 = 5f32;
    const TARGET_TEXT_POS: f32 = 72f32;
    const INITIAL_TEXT_POS: f32 = -384f32;

    pub fn new() -> Self {
        Self {
            menu_state: MenuState::Idle,
            // Idle
            main_menu_activity: MenuActivity::Show,
            activity_direction_right: false,
            chosen_index: 0u8,
            text_pos_x: Self::INITIAL_TEXT_POS,
            text_pos_x_mod: 32f32,
            timer_activity: Self::ACTIVITY_TIME_MIN,
            // Option
            option_activity: MenuActivity::Hide,
        }
    }

    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        gd: &mut GameData,
        delta_time: &f32,
        cam: &mut Camera3D,
        game_state: &mut GameState,
    ) {
        // exammple update
        rl.update_camera(cam, CameraMode::CAMERA_ORBITAL);

        match self.menu_state {
            MenuState::Idle => {
                self.handle_idle_update(rl, gd, delta_time);
            }
            MenuState::Start => {}
            MenuState::StartExtra => {}
            MenuState::StartPractice => {}
            MenuState::Score => {}
            MenuState::Option => {
                self.handle_option_update(rl, gd, delta_time);
            }
            MenuState::Quit => {
                gd.window_must_close();
            }
        }
    }

    pub fn draw(
        &self,
        thread: &RaylibThread,
        rl: &mut RaylibHandle,
        font: &Font,
        cam: &mut Camera3D,
        render_target: &mut RenderTexture2D,
    ) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::BLACK);
        // Example Text
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
            let mut d = d.begin_texture_mode(thread, render_target);
            d.clear_background(Color::DIMGRAY);
            // Example Text
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
                match self.menu_state {
                    MenuState::Idle => {
                        const FONT_SIZE: f32 = 84f32;
                        const TEXT_GAP: f32 = 72f32;
                        const INACTIVE_WHITE: Color = Color::new(255u8, 255u8, 255u8, 191u8);
                        const TEXT_POSITION: f32 = SCREEN_HEIGHT as f32 - 32f32;

                        d.draw_text_ex(
                            font,
                            "Start",
                            Vector2::new(
                                self.text_pos_x - self.text_pos_x_mod * 0.2f32 + 5f32,
                                TEXT_POSITION as f32 - TEXT_GAP * 6f32,
                            ),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 0 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Start Extra",
                            Vector2::new(
                                self.text_pos_x - self.text_pos_x_mod * 1.2f32,
                                TEXT_POSITION as f32 - TEXT_GAP * 5f32,
                            ),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 1 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Start Practice",
                            Vector2::new(
                                self.text_pos_x + self.text_pos_x_mod * 0.3f32,
                                TEXT_POSITION as f32 - TEXT_GAP * 4f32,
                            ),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 2 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Score",
                            Vector2::new(
                                self.text_pos_x - self.text_pos_x_mod * 0.4f32,
                                TEXT_POSITION as f32 - TEXT_GAP * 3f32,
                            ),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 3 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Option",
                            Vector2::new(
                                self.text_pos_x + self.text_pos_x_mod * 1.2f32,
                                TEXT_POSITION as f32 - TEXT_GAP * 2f32,
                            ),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 4 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Quit",
                            Vector2::new(self.text_pos_x + 20f32, TEXT_POSITION as f32 - TEXT_GAP),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 5 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                    }
                    MenuState::Start => {}
                    MenuState::StartExtra => {}
                    MenuState::StartPractice => {}
                    MenuState::Score => {}
                    MenuState::Option => {
                        const FONT_SIZE: f32 = 84f32;
                        const TEXT_GAP: f32 = 72f32;
                        const INACTIVE_WHITE: Color = Color::new(255u8, 255u8, 255u8, 191u8);
                        const INACTIVE_WHITE_MAIN: Color = Color::new(255u8, 232u8, 232u8, 232u8);
                        const TEXT_POSITION: f32 = SCREEN_HEIGHT as f32 - 32f32;
                        // MODE: Fullscreen Windowed
                        {
                            d.draw_text_ex(
                                font,
                                "Windowed",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 8f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 0 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "Fullscreen",
                                Vector2::new(
                                    self.text_pos_x + 340f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 8f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 0 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        // FPS
                        {
                            d.draw_text_ex(
                                font,
                                "FPS",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 7f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 1 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "On",
                                Vector2::new(
                                    self.text_pos_x + 340f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 7f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 1 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "Off",
                                Vector2::new(
                                    self.text_pos_x + 440f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 7f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 1 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        // V-Sync
                        {
                            d.draw_text_ex(
                                font,
                                "V-Sync",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 6f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 2 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "On",
                                Vector2::new(
                                    self.text_pos_x + 340f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 6f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 2 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "Off",
                                Vector2::new(
                                    self.text_pos_x + 440f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 6f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 2 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        // BGM
                        {
                            d.draw_text_ex(
                                font,
                                "BGM",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 5f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 3 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "no audio",
                                Vector2::new(
                                    self.text_pos_x + 340f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 5f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 3 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        // SFX
                        {
                            d.draw_text_ex(
                                font,
                                "SFX",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 4f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 4 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "no audio",
                                Vector2::new(
                                    self.text_pos_x + 340f32,
                                    TEXT_POSITION as f32 - TEXT_GAP * 4f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 4 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        d.draw_text_ex(
                            font,
                            "Configure Keys",
                            Vector2::new(
                                self.text_pos_x - 40f32,
                                TEXT_POSITION as f32 - TEXT_GAP * 3f32,
                            ),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 5 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Reset",
                            Vector2::new(
                                self.text_pos_x - 40f32,
                                TEXT_POSITION as f32 - TEXT_GAP * 2f32,
                            ),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 6 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Back",
                            Vector2::new(self.text_pos_x - 40f32, TEXT_POSITION as f32 - TEXT_GAP),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 7 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                    }
                    MenuState::Quit => {
                        // Essentially nothing
                    }
                }
            }
        }
        draw_on_target(&mut d, render_target);
    }

    // MAIN
    fn handle_idle_update(&mut self, rl: &RaylibHandle, gd: &mut GameData, delta_time: &f32) {
        match self.main_menu_activity {
            MenuActivity::Show => {
                const TEXT_MOVING_SPEED_INIT: f32 = 4096f32;

                if self.text_pos_x < Self::TARGET_TEXT_POS {
                    self.text_pos_x += TEXT_MOVING_SPEED_INIT * delta_time;
                } else {
                    self.main_menu_activity = MenuActivity::Idle;
                }
            }
            MenuActivity::Idle => {
                const MAX_POS_MOD: f32 = 32f32;
                const TEXT_MOVING_SPEED: f32 = 128f32;

                // Handle Idle movement
                {
                    if self.timer_activity > 0f32 {
                        self.timer_activity -= delta_time;
                    } else if self.activity_direction_right {
                        self.text_pos_x_mod += TEXT_MOVING_SPEED * delta_time;
                        if self.text_pos_x_mod >= MAX_POS_MOD {
                            self.activity_direction_right = false;
                            self.timer_activity = rand::random_range(
                                Self::ACTIVITY_TIME_MIN..Self::ACTIVITY_TIME_MAX,
                            );
                        }
                    } else if !self.activity_direction_right {
                        self.text_pos_x_mod -= TEXT_MOVING_SPEED * delta_time;
                        if self.text_pos_x_mod <= -MAX_POS_MOD {
                            self.activity_direction_right = true;
                            self.timer_activity = rand::random_range(
                                Self::ACTIVITY_TIME_MIN..Self::ACTIVITY_TIME_MAX,
                            );
                        }
                    }
                }

                // HANDLE INPUT
                {
                    if rl.is_key_pressed(gd.key("down")) {
                        if self.chosen_index == 5u8 {
                            self.chosen_index = 0u8;
                        } else {
                            self.chosen_index += 1;
                        }
                    }
                    if rl.is_key_pressed(gd.key("up")) {
                        if self.chosen_index == 0u8 {
                            self.chosen_index = 5u8;
                        } else {
                            self.chosen_index -= 1;
                        }
                    }
                    if rl.is_key_pressed(REJECT) || rl.is_key_pressed(gd.key("bomb")) {
                        self.chosen_index = 5u8;
                    }
                }

                // HANDLE CHOISE
                {
                    if rl.is_key_pressed(ACCEPT) || rl.is_key_pressed(gd.key("attack")) {
                        match self.chosen_index {
                            0 => {}
                            1 => {}
                            2 => {}
                            3 => {}
                            4 => {
                                //TODO: next_menu_state MUST be introduced
                                self.main_menu_activity = MenuActivity::Hide;
                            }
                            5 => {
                                self.menu_state = MenuState::Quit;
                            }
                            _ => self.chosen_index = 0,
                        }
                    }
                }
            }
            MenuActivity::Hide => {
                const TEXT_MOVING_SPEED_INIT: f32 = 4096f32;

                if self.text_pos_x > Self::INITIAL_TEXT_POS {
                    self.text_pos_x -= TEXT_MOVING_SPEED_INIT * delta_time;
                } else {
                    self.chosen_index = 0;
                    self.option_activity = MenuActivity::Show;
                    self.menu_state = MenuState::Option;
                }
            }
        }
    }

    // OPTION
    fn handle_option_update(&mut self, rl: &RaylibHandle, gd: &mut GameData, delta_time: &f32) {
        let next_menu_state: MenuState;
        match self.option_activity {
            MenuActivity::Show => {
                const TEXT_MOVING_SPEED_INIT: f32 = 4096f32;

                if self.text_pos_x < Self::TARGET_TEXT_POS {
                    self.text_pos_x += TEXT_MOVING_SPEED_INIT * delta_time;
                } else {
                    self.option_activity = MenuActivity::Idle;
                }
            }
            MenuActivity::Idle => {
                // HANDLE INPUT
                {
                    if rl.is_key_pressed(gd.key("down")) {
                        if self.chosen_index == 7u8 {
                            self.chosen_index = 0u8;
                        } else {
                            self.chosen_index += 1;
                        }
                    }
                    if rl.is_key_pressed(gd.key("up")) {
                        if self.chosen_index == 0u8 {
                            self.chosen_index = 7u8;
                        } else {
                            self.chosen_index -= 1;
                        }
                    }
                    if rl.is_key_pressed(REJECT) || rl.is_key_pressed(gd.key("bomb")) {
                        self.chosen_index = 7u8;
                    }
                }

                // HANDLE CHOISE
                {
                    if rl.is_key_pressed(ACCEPT) || rl.is_key_pressed(gd.key("attack")) {
                        match self.chosen_index {
                            0 => {
                                // Windowed / Fullscreen
                            }
                            1 => {
                                // Draw FPS or not
                            }
                            2 => {
                                // V-Sync
                            }
                            3 => {
                                // BGM
                            }
                            4 => {
                                // SFX
                            }
                            5 => {
                                // Configure Keys
                            }
                            6 => {
                                // Reset
                            }
                            7 => {
                                // Consider adding a next state variable if more tree menus will be done
                                self.option_activity = MenuActivity::Hide;
                            }
                            _ => self.chosen_index = 0,
                        }
                    }
                }
            }
            MenuActivity::Hide => {
                const TEXT_MOVING_SPEED_INIT: f32 = 4096f32;

                if self.text_pos_x > Self::INITIAL_TEXT_POS {
                    self.text_pos_x -= TEXT_MOVING_SPEED_INIT * delta_time;
                } else {
                    self.chosen_index = 4;
                    self.main_menu_activity = MenuActivity::Show;
                    self.menu_state = MenuState::Idle;
                }
            }
        }
    }
}
