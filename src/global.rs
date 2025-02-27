use raylib::prelude::*;

// CONSTANTS
pub const MAX_FPS: u32 = 144;
pub const SCREEN_HEIGHT: i32 = 960;
pub const SCREEN_WIDTH: i32 = 720;
pub const MAIN_FONT: &[u8; 46020] = include_bytes!("../fonts/Catholicon.ttf");

// INPUT KEYS
pub const ACCEPT: KeyboardKey = KeyboardKey::KEY_ENTER;
pub const REJECT: KeyboardKey = KeyboardKey::KEY_BACKSPACE;

// GAMESTATES
pub enum GameState {
    GreetingScreen, // Press enter
    MainMenu,       // Menu start quit settings, etc
    Playing,        // GameLoop goes / pause here
    GameOver,       // Player lost all lifes
    EndScreen,      // Player won and titles are shown
}

// GLOBAL DATA
pub struct GameData {
    // Window vars
    window_should_close: bool,
    window_fullscreen: bool,
    should_draw_fps: bool,

    // GameKeys
    up: KeyboardKey,
    down: KeyboardKey,
    left: KeyboardKey,
    right: KeyboardKey,
    attack: KeyboardKey,
    bomb: KeyboardKey,
    slow: KeyboardKey,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            // Window
            window_should_close: false,
            window_fullscreen: false,
            should_draw_fps: true,

            // Keys
            up: KeyboardKey::KEY_UP,
            down: KeyboardKey::KEY_DOWN,
            left: KeyboardKey::KEY_LEFT,
            right: KeyboardKey::KEY_RIGHT,
            attack: KeyboardKey::KEY_Z,
            bomb: KeyboardKey::KEY_X,
            slow: KeyboardKey::KEY_LEFT_SHIFT,
        }
    }

    /* Window */
    /// Window will close on next iteration if true
    pub fn window_must_close(&mut self) {
        self.window_should_close = true;
    }

    /// Returns true if window must close
    pub fn window_should_close(&self) -> bool {
        self.window_should_close
    }

    /* FPS */
    /// Toggles is fps should be drawn
    pub fn fps_should_draw_toggle(&mut self) {
        if self.should_draw_fps == true {
            self.should_draw_fps = false;
        } else {
            self.should_draw_fps = true;
        }
    }

    /// Returns if fps should be drawn
    pub fn fps_should_draw(&self) -> bool {
        self.should_draw_fps
    }

    /// Toggle Fullscreen using gamedata, returns fullscreen state in written in Data
    pub fn toggle_fullscreen(&mut self) -> bool {
        if self.window_fullscreen == true {
            self.window_fullscreen = false;
        } else {
            self.window_fullscreen = true;
        }
        self.window_fullscreen
    }

    pub fn is_fullscreen(&self) -> bool {
        self.window_fullscreen
    }

    /// Keys Data loaded in gamedata. Provide with action: "up", "down", "left", "right", "attack", "bomb", "slow"
    pub fn key(&self, action: &str) -> KeyboardKey {
        match action {
            "up" => self.up,
            "down" => self.down,
            "left" => self.left,
            "right" => self.right,
            "attack" => self.attack,
            "bomb" => self.bomb,
            "slow" => self.slow,
            _ => panic!(
                "Action '{}' does not exist! Refer to list of actions in the global.rs file",
                action
            ),
        }
    }

    // TODO: Implement saving configs into settings.dat
    /// Update KeyData from settings
    pub fn set_key(&mut self, action: &str, new_key: KeyboardKey) {
        match action {
            "up" => self.up = new_key,
            "down" => self.down = new_key,
            "left" => self.left = new_key,
            "right" => self.right = new_key,
            "attack" => self.attack = new_key,
            "bomb" => self.bomb = new_key,
            "slow" => self.slow = new_key,
            _ => panic!("Action '{}' does not exist!", action),
        }
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

/// lerp that handles appropriate error, so no infinite interpolation
pub fn lerp_e(v0: f32, v1: f32, amount: f32, err_size: f32) -> f32 {
    if !compare_floats(v0, v1, err_size) {
        return v0 + amount * (v1 - v0);
    } else {
        v0
    }
}

/// compares if floats are almost equal, and returns true if they are
pub fn compare_floats(v0: f32, v1: f32, err_size: f32) -> bool {
    if (v0 - v1).abs() > err_size {
        false
    } else {
        true
    }
}
