use raylib::prelude::*;

use crate::global::*;

pub struct MainMenu {
    menu_state: MenuState,
    timer_activity: f32,
}

enum MenuState {
    Idle,   // Default
    Start,
    StartExtra,
    StartPractice,
    Score,
    Option,
    Quit
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            menu_state: MenuState::Idle,
            timer_activity: 0f32,
        }
    }
    
    pub fn update(&mut self, rl: &mut RaylibHandle, delta_time: &f32, cam: &mut Camera3D, game_state: &mut GameState) {
        // exammple update
        rl.update_camera(cam, CameraMode::CAMERA_ORBITAL);
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
        }
        draw_on_target(&mut d, &render_target);
    }
}
