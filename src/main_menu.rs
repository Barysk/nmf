use raylib::prelude::*;

use crate::global::*;

pub struct MainMenu {
    menu_state: MenuState,
    next_menu_state: MenuState,
    current_activity: MenuActivity,
    // Main menu things
    activity_direction_right: bool,
    text_pos_x: f32,
    text_pos_x_mod: f32,
    timer_activity: f32,
    chosen_index: u8,
    // Option things
    dot_position: Vector2,
    // KBD Option Settings
    is_listening: bool,
    // ...
}

enum MenuActivity {
    Show,
    Idle,
    Hide,
}

#[derive(Clone, Copy, PartialEq)]
enum MenuState {
    Idle,          // Default
    Start,         // 2
    StartExtra,    // 6
    StartPractice, // 3
    Score,         // 5
    Option,        // 1
    OptionKBD,     // 1.2
    Quit,          // Quit
}

impl MainMenu {
    const ACTIVITY_TIME_MIN: f32 = 3f32;
    const ACTIVITY_TIME_MAX: f32 = 5f32;
    const TARGET_TEXT_POS: f32 = 96f32;
    const INITIAL_TEXT_POS: f32 = -380f32;
    const LERP_SPEED: f32 = 24f32;
    const LERP_SPEED_ACTIVITY: f32 = 4f32;
    const LERP_ACCEPTABLE_ERR: f32 = 0.8f32;

    pub fn new() -> Self {
        Self {
            menu_state: MenuState::Idle,
            next_menu_state: MenuState::Idle,
            current_activity: MenuActivity::Show,
            // Idle
            activity_direction_right: false,
            chosen_index: 0u8,
            text_pos_x: Self::INITIAL_TEXT_POS,
            text_pos_x_mod: 32f32,
            timer_activity: Self::ACTIVITY_TIME_MIN,
            // Option
            dot_position: Vector2::new(Self::INITIAL_TEXT_POS, 0f32),
            // Option KDB
            is_listening: false,
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
            MenuState::OptionKBD => {
                // TODO: Update OptionKBD
                self.handle_option_kbd_update(rl, gd, delta_time);
            }
            MenuState::Quit => {
                gd.window_must_close();
            }
        }
    }

    pub fn draw(
        &self,
        thread: &RaylibThread,
        d: &mut RaylibDrawHandle,
        gd: &GameData,
        font: &Font,
        cam: &Camera3D,
        render_target: &mut RenderTexture2D,
    ) {
        // DRAW IN VIEWPORT
        {
            let mut d = d.begin_texture_mode(thread, render_target);
            d.clear_background(Color::DIMGRAY);
            // Example Text
            d.draw_text_ex(
                font,
                "Menu",
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
                                TEXT_POSITION - TEXT_GAP * 6f32,
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
                                TEXT_POSITION - TEXT_GAP * 5f32,
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
                                TEXT_POSITION - TEXT_GAP * 4f32,
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
                                TEXT_POSITION - TEXT_GAP * 3f32,
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
                                TEXT_POSITION - TEXT_GAP * 2f32,
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
                            Vector2::new(self.text_pos_x + 20f32, TEXT_POSITION - TEXT_GAP),
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
                        const TEXT_POSITION: f32 = SCREEN_HEIGHT as f32 - 32f32;
                        d.draw_circle_v(self.dot_position, 8f32, Color::WHITE);
                        // MODE: Fullscreen Windowed
                        {
                            d.draw_text_ex(
                                font,
                                "Windowed",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 9f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if !gd.is_fullscreen() {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "Fullscreen",
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 9f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if gd.is_fullscreen() {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        // FPS cap
                        {
                            d.draw_text_ex(
                                font,
                                "FPS Limit",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 8f32,
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
                                format!("{}", gd.get_max_fps()).as_str(),
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 8f32,
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
                        // Show FPS
                        {
                            d.draw_text_ex(
                                font,
                                "Show FPS",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 7f32,
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
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 7f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if gd.fps_should_draw() {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "Off",
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod + 100f32,
                                    TEXT_POSITION - TEXT_GAP * 7f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if !gd.fps_should_draw() {
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
                                    TEXT_POSITION - TEXT_GAP * 6f32,
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
                                "On",
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 6f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if gd.is_vsync_enabled() {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            d.draw_text_ex(
                                font,
                                "Off",
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod + 100f32,
                                    TEXT_POSITION - TEXT_GAP * 6f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if !gd.is_vsync_enabled() {
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
                                    TEXT_POSITION - TEXT_GAP * 5f32,
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
                                format!("{}%", gd.get_bgm_volume_prc()).as_str(),
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 5f32,
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
                        // SFX
                        {
                            d.draw_text_ex(
                                font,
                                "SFX",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 4f32,
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
                                format!("{}%", gd.get_sfx_volume_prc()).as_str(),
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 4f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 5 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        d.draw_text_ex(
                            font,
                            "Configure Keys", // TODO: Configure Keys entry
                            Vector2::new(self.text_pos_x - 40f32, TEXT_POSITION - TEXT_GAP * 3f32),
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
                            "Reset",
                            Vector2::new(self.text_pos_x - 40f32, TEXT_POSITION - TEXT_GAP * 2f32),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 7 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                        d.draw_text_ex(
                            font,
                            "Back",
                            Vector2::new(self.text_pos_x - 40f32, TEXT_POSITION - TEXT_GAP),
                            FONT_SIZE,
                            1f32,
                            if self.chosen_index == 8 {
                                Color::WHITE
                            } else {
                                INACTIVE_WHITE
                            },
                        );
                    }
                    MenuState::OptionKBD => {
                        // FIXME: Drawing OptionKBD
                        const FONT_SIZE: f32 = 84f32;
                        const TEXT_GAP: f32 = 72f32;
                        const INACTIVE_WHITE: Color = Color::new(255u8, 255u8, 255u8, 191u8);
                        const TEXT_POSITION: f32 = SCREEN_HEIGHT as f32 - 32f32;
                        d.draw_circle_v(self.dot_position, 8f32, Color::WHITE);
                        // var used for dynamic key naming
                        let mut key_str: String;

                        // UP
                        {
                            // drawing
                            d.draw_text_ex(
                                font,
                                "Move up",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 8f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 0 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );

                            key_str = gd.get_key_as_string(gd.key("up"));
                            d.draw_text_ex(
                                font,
                                if self.is_listening && self.chosen_index == 0 {
                                    "Listening"
                                } else {
                                    &key_str
                                },
                                // "BTN",  // TODO: if listening than write listening
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 8f32,
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
                        // Down
                        {
                            d.draw_text_ex(
                                font,
                                "Move down",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 7f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 1 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            key_str = gd.get_key_as_string(gd.key("down"));
                            d.draw_text_ex(
                                font,
                                if self.is_listening && self.chosen_index == 1 {
                                    "Listening"
                                } else {
                                    &key_str
                                },
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 7f32,
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
                        // Left
                        {
                            d.draw_text_ex(
                                font,
                                "Move left",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 6f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 2 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            key_str = gd.get_key_as_string(gd.key("left"));
                            d.draw_text_ex(
                                font,
                                if self.is_listening && self.chosen_index == 2 {
                                    "Listening"
                                } else {
                                    &key_str
                                },
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 6f32,
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
                        // Right
                        {
                            d.draw_text_ex(
                                font,
                                "Move right",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 5f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 3 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            key_str = gd.get_key_as_string(gd.key("right"));
                            d.draw_text_ex(
                                font,
                                if self.is_listening && self.chosen_index == 3 {
                                    "Listening"
                                } else {
                                    &key_str
                                },
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 5f32,
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
                        // Attack
                        {
                            d.draw_text_ex(
                                font,
                                "Attack",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 4f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 4 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            key_str = gd.get_key_as_string(gd.key("attack"));
                            d.draw_text_ex(
                                font,
                                if self.is_listening && self.chosen_index == 4 {
                                    "Listening"
                                } else {
                                    &key_str
                                },
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 4f32,
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
                        // Bomb
                        {
                            d.draw_text_ex(
                                font,
                                "Bomb",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 3f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 5 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            key_str = gd.get_key_as_string(gd.key("bomb"));
                            d.draw_text_ex(
                                font,
                                if self.is_listening && self.chosen_index == 5 {
                                    "Listening"
                                } else {
                                    &key_str
                                },
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 3f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 5 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        // Slow
                        {
                            d.draw_text_ex(
                                font,
                                "Slow",
                                Vector2::new(
                                    self.text_pos_x - 40f32,
                                    TEXT_POSITION - TEXT_GAP * 2f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 6 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                            key_str = gd.get_key_as_string(gd.key("slow"));
                            d.draw_text_ex(
                                font,
                                if self.is_listening && self.chosen_index == 6 {
                                    "Listening"
                                } else {
                                    &key_str
                                },
                                Vector2::new(
                                    self.text_pos_x + self.text_pos_x_mod,
                                    TEXT_POSITION - TEXT_GAP * 2f32,
                                ),
                                FONT_SIZE,
                                1f32,
                                if self.chosen_index == 6 {
                                    Color::WHITE
                                } else {
                                    INACTIVE_WHITE
                                },
                            );
                        }
                        d.draw_text_ex(
                            font,
                            "Back",
                            Vector2::new(self.text_pos_x - 40f32, TEXT_POSITION - TEXT_GAP),
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
        draw_on_target(d, render_target);
    }

    // MAIN
    fn handle_idle_update(&mut self, rl: &RaylibHandle, gd: &mut GameData, delta_time: &f32) {
        match self.current_activity {
            MenuActivity::Show => {
                if self.text_pos_x < Self::TARGET_TEXT_POS {
                    self.text_pos_x = lerp_e(
                        self.text_pos_x,
                        Self::TARGET_TEXT_POS,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                } else {
                    self.text_pos_x = Self::TARGET_TEXT_POS;
                    self.current_activity = MenuActivity::Idle;
                }
            }
            MenuActivity::Idle => {
                const MAX_POS_MOD: f32 = 32f32;

                // Handle Idle movement
                {
                    if self.timer_activity > 0f32 {
                        self.timer_activity -= delta_time;
                    } else if self.activity_direction_right {
                        self.text_pos_x_mod = lerp_e(
                            self.text_pos_x_mod,
                            MAX_POS_MOD,
                            delta_time,
                            Self::LERP_SPEED_ACTIVITY,
                            Self::LERP_ACCEPTABLE_ERR,
                        );
                        if self.text_pos_x_mod >= MAX_POS_MOD {
                            self.activity_direction_right = false;
                            self.timer_activity = rand::random_range(
                                Self::ACTIVITY_TIME_MIN..Self::ACTIVITY_TIME_MAX,
                            );
                        }
                    } else if !self.activity_direction_right {
                        self.text_pos_x_mod = lerp_e(
                            self.text_pos_x_mod,
                            -MAX_POS_MOD,
                            delta_time,
                            Self::LERP_SPEED_ACTIVITY,
                            Self::LERP_ACCEPTABLE_ERR,
                        );
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
                                self.current_activity = MenuActivity::Hide;
                                self.next_menu_state = MenuState::Option;
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
                if self.text_pos_x > Self::INITIAL_TEXT_POS {
                    self.text_pos_x = lerp_e(
                        self.text_pos_x,
                        Self::INITIAL_TEXT_POS,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                } else {
                    self.chosen_index = 0;
                    self.text_pos_x_mod = 0f32;
                    self.current_activity = MenuActivity::Show;
                    self.menu_state = self.next_menu_state;
                }
            }
        }
    }

    // OPTION
    fn handle_option_update(&mut self, rl: &mut RaylibHandle, gd: &mut GameData, delta_time: &f32) {
        const TEXT_GAP: f32 = 72f32;
        const TEXT_POSITION: f32 = SCREEN_HEIGHT as f32 - 32f32;
        const MOD_TEXT_POSITION: f32 = 340f32;
        const LERP_NAVDOT: f32 = 16f32;

        match self.current_activity {
            MenuActivity::Show => {
                // Move text on the specified positions
                if self.text_pos_x < Self::TARGET_TEXT_POS
                    && self.text_pos_x_mod < MOD_TEXT_POSITION
                {
                    self.text_pos_x = lerp_e(
                        self.text_pos_x,
                        Self::TARGET_TEXT_POS,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                    self.text_pos_x_mod = lerp_e(
                        self.text_pos_x_mod,
                        MOD_TEXT_POSITION,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                } else {
                    self.text_pos_x = Self::TARGET_TEXT_POS;
                    self.current_activity = MenuActivity::Idle;
                }

                // Handle appearing of the NAV DOT in right place
                self.dot_position.x = self.text_pos_x - 72f32;
                self.dot_position.y = (TEXT_POSITION + 40f32) - (TEXT_GAP * 9f32)
                    + (TEXT_GAP * self.chosen_index as f32);
            }
            MenuActivity::Idle => {
                // Move NAV DOT till on y axis using interpolation_err
                self.dot_position.y = lerp_e(
                    self.dot_position.y,
                    (TEXT_POSITION + 40f32) - (TEXT_GAP * 9f32)
                        + (TEXT_GAP * self.chosen_index as f32),
                    delta_time,
                    LERP_NAVDOT,
                    Self::LERP_ACCEPTABLE_ERR,
                );

                // HANDLE INPUT
                {
                    if rl.is_key_pressed(gd.key("down")) {
                        if self.chosen_index == 8u8 {
                            self.chosen_index = 0u8;
                        } else {
                            self.chosen_index += 1;
                        }
                    }
                    if rl.is_key_pressed(gd.key("up")) {
                        if self.chosen_index == 0u8 {
                            self.chosen_index = 8u8;
                        } else {
                            self.chosen_index -= 1;
                        }
                    }
                    if rl.is_key_pressed(REJECT) || rl.is_key_pressed(gd.key("bomb")) {
                        self.chosen_index = 8u8;
                    }
                }

                // HANDLE INPUT
                {
                    if rl.is_key_pressed(gd.key("left")) || rl.is_key_pressed(gd.key("right")) {
                        match self.chosen_index {
                            0 => {
                                // Windowed / Fullscreen
                                gd.toggle_fullscreen(rl);
                            }
                            1 => {
                                let current_max_fps = gd.get_max_fps();
                                if rl.is_key_pressed(gd.key("left")) && current_max_fps > 30u32 {
                                    gd.set_max_fps(rl, current_max_fps - 12u32);
                                }
                                if rl.is_key_pressed(gd.key("right")) && current_max_fps < 480 {
                                    gd.set_max_fps(rl, current_max_fps + 12u32);
                                }
                            }
                            2 => {
                                // FPS
                                gd.fps_should_draw_toggle();
                            }
                            3 => {
                                // V-Sync
                                gd.toggle_vsync(rl);
                            }
                            4 => {
                                // BGM
                                // TODO: Actually Change BGM volume
                                if rl.is_key_pressed(gd.key("left")) {
                                    let bgm_volume: f32 = gd.get_bgm_volume() - 0.1f32;
                                    //gd.set_bgm_volume(bgm_audio, bgm_volume.clamp(0f32, 1f32));
                                    gd.set_bgm_volume(bgm_volume.clamp(0f32, 1f32));
                                }
                                if rl.is_key_pressed(gd.key("right")) {
                                    let bgm_volume: f32 = gd.get_bgm_volume() + 0.1f32;
                                    gd.set_bgm_volume(bgm_volume.clamp(0f32, 1f32));
                                }
                            }
                            5 => {
                                // SFX
                                // TODO: Actually Change SFX colume
                                if rl.is_key_pressed(gd.key("left")) {
                                    let sfx_volume: f32 = gd.get_sfx_volume() - 0.1f32;
                                    gd.set_sfx_volume(sfx_volume.clamp(0f32, 1f32));
                                }
                                if rl.is_key_pressed(gd.key("right")) {
                                    let sfx_volume: f32 = gd.get_sfx_volume() + 0.1f32;
                                    gd.set_sfx_volume(sfx_volume.clamp(0f32, 1f32));
                                }
                            }
                            _ => {}
                        }
                    }

                    if rl.is_key_pressed(ACCEPT) || rl.is_key_pressed(gd.key("attack")) {
                        match self.chosen_index {
                            6 => {
                                // Configure Keys
                                self.current_activity = MenuActivity::Hide;
                                self.next_menu_state = MenuState::OptionKBD;
                            }
                            7 => {
                                // Reset
                                gd.reset_options(rl);
                            }
                            8 => {
                                self.current_activity = MenuActivity::Hide;
                                self.next_menu_state = MenuState::Idle;
                            }
                            _ => {}
                        }
                    }
                }
            }
            MenuActivity::Hide => {
                // Hiding Texts
                if self.text_pos_x > Self::INITIAL_TEXT_POS && self.text_pos_x_mod > 0f32 {
                    self.text_pos_x = lerp_e(
                        self.text_pos_x,
                        Self::INITIAL_TEXT_POS,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                    self.text_pos_x_mod = lerp_e(
                        self.text_pos_x_mod,
                        0f32,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                } else {
                    if self.next_menu_state == MenuState::Idle {
                        self.chosen_index = 4;
                    } else {
                        self.chosen_index = 0;
                    }
                    self.current_activity = MenuActivity::Show;
                    self.menu_state = self.next_menu_state;
                    {
                        // resetting those values to reuse them
                        self.text_pos_x_mod = 32f32;
                        self.activity_direction_right = false;
                        self.timer_activity = Self::ACTIVITY_TIME_MIN;
                    }
                }

                // Move NAV DOT till on x axis
                self.dot_position.x = self.text_pos_x - 72f32;
            }
        }
    }

    fn handle_option_kbd_update(
        &mut self,
        rl: &mut RaylibHandle,
        gd: &mut GameData,
        delta_time: &f32,
    ) {
        const TEXT_GAP: f32 = 72f32;
        const TEXT_POSITION: f32 = SCREEN_HEIGHT as f32 - 32f32;
        const MOD_TEXT_POSITION: f32 = 280f32;
        const LERP_NAVDOT: f32 = 16f32;

        match self.current_activity {
            MenuActivity::Show => {
                // Move text on the specified positions
                if self.text_pos_x < Self::TARGET_TEXT_POS
                    && self.text_pos_x_mod < MOD_TEXT_POSITION
                {
                    self.text_pos_x = lerp_e(
                        self.text_pos_x,
                        Self::TARGET_TEXT_POS,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                    self.text_pos_x_mod = lerp_e(
                        self.text_pos_x_mod,
                        MOD_TEXT_POSITION,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                } else {
                    self.text_pos_x = Self::TARGET_TEXT_POS;
                    self.current_activity = MenuActivity::Idle;
                }

                // Handle appearing of the NAV DOT in right place
                self.dot_position.x = self.text_pos_x - 72f32;
                self.dot_position.y = (TEXT_POSITION + 40f32) - (TEXT_GAP * 8f32)
                    + (TEXT_GAP * self.chosen_index as f32);
            }
            MenuActivity::Idle => {
                // Move NAV DOT till on y axis using interpolation_err
                self.dot_position.y = lerp_e(
                    self.dot_position.y,
                    (TEXT_POSITION + 40f32) - (TEXT_GAP * 8f32)
                        + (TEXT_GAP * self.chosen_index as f32),
                    delta_time,
                    LERP_NAVDOT,
                    Self::LERP_ACCEPTABLE_ERR,
                );

                // HANDLE INPUT
                if !self.is_listening {
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
                
                    // HANDLE INPUT
                    // TODO: Sets listening to change a button
                    if rl.is_key_pressed(ACCEPT) || rl.is_key_pressed(gd.key("attack")) {
                        match self.chosen_index {
                            0..=6 => self.is_listening = true,
                            7 => {
                                self.current_activity = MenuActivity::Hide;
                                self.next_menu_state = MenuState::Option;
                            }
                            _ => {}
                        }
                    }
                }

                // HANDLE LISTENING
                if self.is_listening {
                    let listened_key: Option<KeyboardKey> = rl.get_key_pressed();

                    if listened_key.is_some() {
                        match self.chosen_index {
                            0 => {
                                gd.set_key("up", listened_key.unwrap());
                                self.is_listening = false
                            }
                            1 => {
                                gd.set_key("down", listened_key.unwrap());
                                self.is_listening = false
                            }
                            2 => {
                                gd.set_key("left", listened_key.unwrap());
                                self.is_listening = false
                            }
                            3 => {
                                gd.set_key("right", listened_key.unwrap());
                                self.is_listening = false
                            }
                            4 => {
                                gd.set_key("attack", listened_key.unwrap());
                                self.is_listening = false
                            }
                            5 => {
                                gd.set_key("bomb", listened_key.unwrap());
                                self.is_listening = false
                            }
                            6 => {
                                gd.set_key("slow", listened_key.unwrap());
                                self.is_listening = false
                            }
                            _ => {
                                self.is_listening = false;
                            }
                        }
                    }
                }
            }
            MenuActivity::Hide => {
                // Hiding Texts
                if self.text_pos_x > Self::INITIAL_TEXT_POS && self.text_pos_x_mod > 0f32 {
                    self.text_pos_x = lerp_e(
                        self.text_pos_x,
                        Self::INITIAL_TEXT_POS,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                    self.text_pos_x_mod = lerp_e(
                        self.text_pos_x_mod,
                        0f32,
                        delta_time,
                        Self::LERP_SPEED,
                        Self::LERP_ACCEPTABLE_ERR,
                    );
                } else {
                    if self.next_menu_state == MenuState::Option {
                        self.chosen_index = 6;
                    } else {
                        self.chosen_index = 0;
                    }
                    self.current_activity = MenuActivity::Show;
                    self.menu_state = self.next_menu_state;
                    {
                        // resetting those values to reuse them
                        self.text_pos_x_mod = 32f32;
                        self.activity_direction_right = false;
                        self.timer_activity = Self::ACTIVITY_TIME_MIN;
                    }
                }

                // Move NAV DOT till on x axis
                self.dot_position.x = self.text_pos_x - 72f32;
            }
        }
    }
}
