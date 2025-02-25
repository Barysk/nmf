use raylib::prelude::*;

// CONSTANTS

pub const MAX_FPS: u32 = 60;
pub const SCREEN_HEIGHT: i32 = 960;
pub const SCREEN_WIDTH: i32 = 720;
pub const MAIN_FONT: &[u8; 46020] = include_bytes!("../fonts/Catholicon.ttf");

// INPUT KEYS
pub const ACCEPT: KeyboardKey = KeyboardKey::KEY_ENTER;
pub const _REJECT: KeyboardKey = KeyboardKey::KEY_BACKSPACE;

// FIXME: are not constants that will be changed by a player, move to GLOBAL_DATA
pub const UP: KeyboardKey = KeyboardKey::KEY_UP;
pub const DOWN: KeyboardKey = KeyboardKey::KEY_DOWN;
pub const _LEFT: KeyboardKey = KeyboardKey::KEY_LEFT;
pub const _RIGHT: KeyboardKey = KeyboardKey::KEY_RIGHT;
pub const ATACK: KeyboardKey = KeyboardKey::KEY_Z;
pub const _BOMB: KeyboardKey = KeyboardKey::KEY_X;
pub const _SLOW: KeyboardKey = KeyboardKey::KEY_LEFT_SHIFT;


// GAMESTATES
pub enum GameState {
    GreetingScreen, // Press enter
    MainMenu,       // Menu start quit settings, etc
    Playing,        // GameLoop goes
    Pause,          // Gameloop poused
    GameOver,       // Player lost all lifes
    EndScreen,      // Player won and titles are shown
}

// GLOBAL DATA
pub struct GameData {
    window_should_close: bool,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            window_should_close: false,
        }
    }

    pub fn window_must_close(&mut self) {
        self.window_should_close = true;
    }

    pub fn window_should_close(&self) -> bool {
        self.window_should_close
    }
}

/// DRAW TEXTURE TARGET
/// Draws viewport saving its dpi
pub fn draw_on_target(d: &mut RaylibDrawHandle, render_target: &RenderTexture2D) {
    // Screen scaling
    let mut scaling: f32 = 1f32;

    let scale_x: f32 = d.get_screen_width() as f32 / SCREEN_WIDTH as f32;
    let scale_y: f32 = d.get_screen_height() as f32 / SCREEN_HEIGHT as f32;

    if scale_x != scaling && scale_y != scaling {
        if scale_x >= scale_y {
            scaling = scale_y;
        } else {
            scaling = scale_x;
        }
    }

    let screen_center: Vector2 = Vector2::new(
        d.get_screen_width() as f32 / 2f32,
        d.get_screen_height() as f32 / 2f32,
    );

    let render_target_position: Vector2 = Vector2::new(
        screen_center.x - render_target.texture.width as f32 * scaling / 2f32,
        screen_center.y - render_target.texture.height as f32 * scaling / 2f32,
    );

    d.draw_texture_pro(
        render_target.texture(),
        rrect(
            0,
            0,
            render_target.texture.width,
            -render_target.texture.height,
        ),
        rrect(
            0,
            0,
            render_target.texture.width as f32 * scaling,
            render_target.texture.height as f32 * scaling,
        ),
        rvec2(-render_target_position.x, -render_target_position.y),
        0f32,
        Color::WHITE,
    );
}
