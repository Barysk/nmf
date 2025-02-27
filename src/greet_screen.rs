use crate::global::*;
use raylib::prelude::*;
pub struct GreetScreen {
    timer_loading: f32,
    is_loaded: bool,
}

impl GreetScreen {
    pub fn new() -> Self {
        Self {
            timer_loading: 0f32,
            is_loaded: false,
        }
    }

    pub fn update(
        &mut self,
        rl: &mut RaylibHandle,
        delta_time: &f32,
        _cam: &mut Camera3D,
        game_state: &mut GameState,
    ) {
        // MIN WAIT TIME
        const WAIT_TIME: f32 = 0.5f32;

        if self.timer_loading < WAIT_TIME && !self.is_loaded {
            self.timer_loading += delta_time;
        } else if self.timer_loading >= WAIT_TIME && !self.is_loaded {
            self.is_loaded = true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) && self.is_loaded {
            *game_state = GameState::MainMenu;
        }
    }

    pub fn draw(
        &self,
        thread: &RaylibThread,
        rl: &mut RaylibHandle,
        font: &Font,
        cam: &Camera3D,
        render_target: &mut RenderTexture2D,
    ) {
        let mut d = rl.begin_drawing(thread);
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
            let mut d = d.begin_texture_mode(thread, render_target);
            d.clear_background(Color::DARKGRAY);
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
            if self.is_loaded {
                const TEXT_POSITION: Vector2 = Vector2::new(140f32, SCREEN_HEIGHT as f32 - 100f32);
                const TEXT_SIZE: f32 = 72f32;
                d.draw_text_ex(
                    font,
                    "Press Enter to start",
                    TEXT_POSITION + 2f32,
                    TEXT_SIZE,
                    1f32,
                    Color::BLACK,
                );
                d.draw_text_ex(
                    font,
                    "Press Enter to start",
                    TEXT_POSITION,
                    TEXT_SIZE,
                    1f32,
                    Color::DARKMAGENTA,
                );
            }
        }
        draw_on_target(&mut d, render_target);
    }
}
