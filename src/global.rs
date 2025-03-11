use raylib::prelude::*;
use std::fs;

// CONSTANTS
pub const SCREEN_HEIGHT: i32 = 960;
pub const SCREEN_WIDTH: i32 = 720;
pub const MAIN_FONT: &[u8; 46020] = include_bytes!("../fonts/Catholicon.ttf");
const OPTIONS_FILE_PATH: &str = "options.dat";

// DEFAULT VALUES
// window
const FULL_SCREEN: bool = false;
const MAX_FPS: u32 = 60u32;
const SHOULD_DRAW_FPS: bool = true;
const VSYNC_ENABLED: bool = false;
const BGM_VOLUME: f32 = 1.0f32;
const SFX_VOLUME: f32 = 1.0f32;
// keys
const UP: KeyboardKey = KeyboardKey::KEY_UP;
const DOWN: KeyboardKey = KeyboardKey::KEY_DOWN;
const LEFT: KeyboardKey = KeyboardKey::KEY_LEFT;
const RIGHT: KeyboardKey = KeyboardKey::KEY_RIGHT;
const ATTACK: KeyboardKey = KeyboardKey::KEY_Z;
const BOMB: KeyboardKey = KeyboardKey::KEY_X;
const SLOW: KeyboardKey = KeyboardKey::KEY_LEFT_SHIFT;

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
    // Options
    window_fullscreen: bool,
    max_fps: u32,
    should_draw_fps: bool,
    vsync_enabled: bool,
    bgm_volume: f32,
    sfx_volume: f32,

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
            // Must not be changed outside window_must_close()
            window_should_close: false,
            // Window
            window_fullscreen: FULL_SCREEN,
            max_fps: MAX_FPS,
            should_draw_fps: SHOULD_DRAW_FPS,
            vsync_enabled: VSYNC_ENABLED, // By default, there is no VSync
            bgm_volume: BGM_VOLUME,
            sfx_volume: SFX_VOLUME,

            // Keys
            up: UP,
            down: DOWN,
            left: LEFT,
            right: RIGHT,
            attack: ATTACK,
            bomb: BOMB,
            slow: SLOW,
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

    /// Toggle Fullscreen using gamedata, returns fullscreen state in written in Data
    pub fn toggle_fullscreen(&mut self, rl: &mut RaylibHandle) {
        self.window_fullscreen = !self.window_fullscreen;
        rl.toggle_borderless_windowed();
        self.save_config();
    }

    /// Returns window_fullscreen, from game data
    pub fn is_fullscreen(&self) -> bool {
        self.window_fullscreen
    }

    /* FPS Cap */
    /// Sets a max frame rate
    pub fn set_max_fps(&mut self, rl: &mut RaylibHandle, new_max_fps: u32) {
        self.max_fps = new_max_fps;
        rl.set_target_fps(self.max_fps);
        self.save_config();
    }

    /// Returns current fps cap
    pub fn get_max_fps(&self) -> u32 {
        self.max_fps
    }

    /* FPS Draw */
    /// Toggles is fps should be drawn
    pub fn fps_should_draw_toggle(&mut self) {
        self.should_draw_fps = !self.should_draw_fps;
        self.save_config();
    }

    /// Returns if fps should be drawn
    pub fn fps_should_draw(&self) -> bool {
        self.should_draw_fps
    }

    /* V-Sync */
    /// Toggle V-Sync
    pub fn toggle_vsync(&mut self, rl: &mut RaylibHandle) {
        self.vsync_enabled = !self.vsync_enabled;
        if self.is_vsync_enabled() {
            rl.set_window_state(WindowState::set_vsync_hint(rl.get_window_state(), true));
        } else {
            rl.clear_window_state(WindowState::set_vsync_hint(rl.get_window_state(), true));
        }
        self.save_config();
    }

    /// Returns true if vsync is enabled
    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync_enabled
    }

    /* Audio */
    /// Set background music volume
    pub fn set_bgm_volume(&mut self, new_volume: f32) {
        if !(0f32..=1f32).contains(&new_volume) {
            panic!(
                "Provided bgm volume {} value is out of bounds [0, 1]",
                new_volume
            )
        }
        self.save_config();
        self.bgm_volume = new_volume
    }

    /// Returns current bgm volume
    pub fn get_bgm_volume(&self) -> f32 {
        self.bgm_volume
    }

    /// Returns current bgm volume in percents
    pub fn get_bgm_volume_prc(&self) -> f32 {
        (self.bgm_volume * 100f32).round()
    }

    /// Set sound effects volume
    pub fn set_sfx_volume(&mut self, new_volume: f32) {
        if !(0f32..=1f32).contains(&new_volume) {
            panic!(
                "Provided sfx volume {} value is out of bounds [0, 1]",
                new_volume
            )
        }
        self.save_config();
        self.sfx_volume = new_volume
    }

    /// Returns sfx volume
    pub fn get_sfx_volume(&self) -> f32 {
        self.sfx_volume
    }

    /// Returns sfx volume in percents
    pub fn get_sfx_volume_prc(&self) -> f32 {
        (self.sfx_volume * 100f32).round()
    }

    /// Resets All gamedata.option values
    pub fn reset_options(&mut self, rl: &mut RaylibHandle) {
        // window
        if self.window_fullscreen != FULL_SCREEN {
            self.toggle_fullscreen(rl);
        }
        if self.max_fps != MAX_FPS {
            self.set_max_fps(rl, MAX_FPS);
        }
        if self.should_draw_fps != SHOULD_DRAW_FPS {
            self.fps_should_draw_toggle();
        }
        if self.vsync_enabled != VSYNC_ENABLED {
            self.toggle_vsync(rl);
        }
        if self.bgm_volume != BGM_VOLUME {
            self.set_bgm_volume(BGM_VOLUME);
        }
        if self.sfx_volume != SFX_VOLUME {
            self.set_sfx_volume(SFX_VOLUME);
        }

        // keys
        self.up = UP;
        self.down = DOWN;
        self.left = LEFT;
        self.right = RIGHT;
        self.attack = ATTACK;
        self.bomb = BOMB;
        self.slow = SLOW;

        rl.set_window_size(320, 320);

        self.save_config();
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

    // FIXME: Make a check, so no important keys go overwritten
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

    pub fn save_config(&self) {
        let option_data: String = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            self.window_fullscreen,
            self.max_fps,
            self.should_draw_fps,
            self.vsync_enabled,
            self.bgm_volume,
            self.sfx_volume,
            self.up as u32,
            self.down as u32,
            self.left as u32,
            self.right as u32,
            self.attack as u32,
            self.bomb as u32,
            self.slow as u32
        );
        fs::write(OPTIONS_FILE_PATH, option_data.as_bytes()).ok();
    }

    // FIXME: All loaded settings must apply, same as in reset 1/2 Keys must have own option screen
    pub fn load_config(&mut self, rl: &mut RaylibHandle) {
        if !fs::exists(OPTIONS_FILE_PATH).unwrap() {
            println!(
                "{} didn't existed so it is being created, with defaults",
                OPTIONS_FILE_PATH
            );
            self.reset_options(rl);
        }
        let option_data: String = fs::read_to_string(OPTIONS_FILE_PATH).unwrap();
        let mut lines = option_data.lines();

        // Window
        self.window_fullscreen = lines.next().unwrap().parse().unwrap();
        self.max_fps = lines.next().unwrap().parse().unwrap();
        self.should_draw_fps = lines.next().unwrap().parse().unwrap();
        self.vsync_enabled = lines.next().unwrap().parse().unwrap(); // By default, there is no VSync
        self.bgm_volume = lines.next().unwrap().parse().unwrap();
        self.sfx_volume = lines.next().unwrap().parse().unwrap();

        // Applying settings
        if self.window_fullscreen != FULL_SCREEN {
            rl.toggle_borderless_windowed();
        }
        if self.max_fps != MAX_FPS {
            self.set_max_fps(rl, self.max_fps);
        }
        if self.should_draw_fps != SHOULD_DRAW_FPS {
            self.should_draw_fps = false;
        }
        if self.vsync_enabled != VSYNC_ENABLED {
            rl.set_window_state(WindowState::set_vsync_hint(rl.get_window_state(), true));
        }
        if self.bgm_volume != BGM_VOLUME {
            self.set_bgm_volume(self.bgm_volume);
        }
        if self.sfx_volume != SFX_VOLUME {
            self.set_sfx_volume(self.sfx_volume);
        }

        // Keys
        self.up = key_from_i32(lines.next().unwrap().parse().unwrap()).unwrap();
        self.down = key_from_i32(lines.next().unwrap().parse().unwrap()).unwrap();
        self.left = key_from_i32(lines.next().unwrap().parse().unwrap()).unwrap();
        self.right = key_from_i32(lines.next().unwrap().parse().unwrap()).unwrap();
        self.attack = key_from_i32(lines.next().unwrap().parse().unwrap()).unwrap();
        self.bomb = key_from_i32(lines.next().unwrap().parse().unwrap()).unwrap();
        self.slow = key_from_i32(lines.next().unwrap().parse().unwrap()).unwrap();
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

/// lerp that handles appropriate error, so no infinite interpolation, also delta time
pub fn lerp_e(v0: f32, v1: f32, delta_time: &f32, lerp_speed: f32, err_size: f32) -> f32 {
    if !compare_floats(v0, v1, err_size) {
        let factor = (lerp_speed * delta_time).min(1f32); // Exponential smoothing
        return v0 + factor * (v1 - v0);
    }
    v1
}

/// compares if floats are almost equal, and returns true if they are
pub fn compare_floats(v0: f32, v1: f32, err_size: f32) -> bool {
    if (v0 - v1).abs() > err_size {
        return false;
    }
    true
}
