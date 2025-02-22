use std::arch::is_aarch64_feature_detected;

use raylib::prelude::*;

// CONSTANTS
const SCREEN_HEIGHT: i32 = 640;
const SCREEN_WIDTH: i32 = 480;
const MAIN_FONT: &[u8; 46020] = include_bytes!("../fonts/Catholicon.ttf");

// GAMESTATES
enum GameState {
    GreetingScreen,   // Press enter
    MainMenu,         // Menu start quit settings, etc
    ChooseDifficulty, // Choosing difficulty after player hit start
    Playing,          // GameLoop goes
    Pause,            // Gameloop poused
    GameOver,         // Player lost all lifes
    EndScreen,        // Player won and titles are shown
}

fn main() {
    // STATE MANAGER
    let mut game_state = GameState::GreetingScreen;

    // INIT WINDOW
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("Noster: Mare Frigoris")
        .build();

    rl.set_window_min_size(240i32, 320i32);
    rl.set_target_fps(60u32);

    let font: Font = rl
        .load_font_from_memory(&thread, ".ttf", MAIN_FONT, 32i32, None)
        .unwrap();

    // INIT CAMERA
    let mut cam = Camera3D::perspective(
        Vector3::new(0f32, 10f32, 10f32),
        Vector3::new(0f32, 0f32, 0f32),
        Vector3::new(0f32, 1f32, 0f32),
        45f32,
    );

    // INIT RENDER TARGET
    let mut render_target: RenderTexture2D = rl
        .load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .unwrap();

    // INIT GREETING SCREEN
    let mut greet_screen: GreetScreen = GreetScreen::new();

    while !rl.window_should_close() {
        // PRE-UPDATE, GLOBAL KEYBOARD INPUT, ETC. | Probably will not be needed
        let delta_time: f32 = rl.get_frame_time();

        // STATE MANAGING
        match game_state {
            // include upd + draw to each state since they have different logic
            // and drawing tasks
            GameState::GreetingScreen => {
                manage_greet(
                    &thread,
                    &mut rl,
                    &mut greet_screen,
                    &delta_time,
                    &font,
                    &mut game_state,
                    &mut cam,
                    &mut render_target,
                );
            }
            GameState::MainMenu => {
                main_menu();
            }
            GameState::ChooseDifficulty => {
                choose_difficulty();
            }
            GameState::Playing => {
                playing();
            }
            GameState::Pause => {
                pause();
            }
            GameState::GameOver => {
                game_over();
            }
            GameState::EndScreen => {
                end_screen();
            }
        }
    }
}

/// DRAW TEXTURE TARGET
/// Draws viewport saving its dpi
fn draw_on_target(d: &mut RaylibDrawHandle, render_target: &RenderTexture2D) {
    // Screen scaling
    let mut scaling: f32 = 1f32;

    let scale_x: f32;
    let scale_y: f32;

    // if d.get_screen_width() >= SCREEN_WIDTH
    // || d.get_screen_height() >= SCREEN_HEIGHT{

    scale_x = d.get_screen_width() as f32 / SCREEN_WIDTH as f32;
    scale_y = d.get_screen_height() as f32 / SCREEN_HEIGHT as f32;

    if scale_x != scaling && scale_y != scaling {
        if scale_x >= scale_y {
            scaling = scale_y as f32;
        } else {
            scaling = scale_x as f32;
        }
    }

    let screen_center: Vector2 = Vector2::new(
        d.get_screen_width() as f32 / 2f32,
        d.get_screen_height() as f32 / 2f32,
    );

    let render_target_position: Vector2 = Vector2::new(
        screen_center.x - (render_target.texture.width as f32 * scaling) as f32 / 2f32,
        screen_center.y - (render_target.texture.height as f32 * scaling) as f32 / 2f32,
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

struct GreetScreen {
    is_color_brightens: bool,
    color: u8,
}

impl GreetScreen {
    fn new() -> Self {
        Self {
            is_color_brightens: true,
            color: 0u8,
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, cam: &mut Camera3D, game_state: &mut GameState) {
        // exammple update
        rl.update_camera(cam, CameraMode::CAMERA_ORBITAL);
        
        if self.is_color_brightens {
            self.color += 5u8;
            if self.color == 255u8 {
                self.is_color_brightens = false;
            }
        } else {
            self.color -= 5u8;
            if self.color == 0u8 {
                self.is_color_brightens = true;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            *game_state = GameState::MainMenu;
        }
    }

    fn draw(
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
            // Color::new(self.color, self.color, self.color, 255u8)
            d.draw_text_ex(
                font,
                "Press Enter to start\n\n[DON'T DO IT NOW]",
                Vector2::new(160f32, 500f32),
                32f32,
                1f32,
                Color::new(self.color, self.color, self.color, 255u8),
            );
        }
        draw_on_target(&mut d, &render_target);
    }
}

/// Handle Greeting Screen State
/// When enter is pressed proceed to main menu and load saves
fn manage_greet(
    thread: &RaylibThread,
    rl: &mut RaylibHandle,
    greet_screen: &mut GreetScreen,
    delta_time: &f32,
    font: &Font,
    game_state: &mut GameState,
    cam: &mut Camera3D,
    render_target: &mut RenderTexture2D,
) {
    greet_screen.update(rl, cam, game_state);
    greet_screen.draw(thread, rl, font, cam, render_target);
}

/// Handle Main Menu State
fn main_menu() {}

/// Handle Choose Difficulty State
fn choose_difficulty() {}

/// Handle GamePlay State
fn playing() {}

/// Handle Pause State
fn pause() {}

/// Handle Game Over State
fn game_over() {}

/// Handle End Screen State
fn end_screen() {}

// // DRAW OUT OF VIEWPORT
// {
//     let mut d = rl.begin_drawing(&thread);
//     d.clear_background(Color::BLACK);
//     //d.draw_text("WHOLE WINDOW", 12, 12, 20, Color::ORANGE);
//     d.draw_text_ex(font, "WHOLE WINDOW", Vector2::new(12f32, 12f32), 22f32, 1f32, Color::ORANGE);

//     // DRAW IN VIEWPORT
//     {
//         let mut d = d.begin_texture_mode(&thread, render_target);
//         d.clear_background(Color::WHITE);
//         d.draw_text_ex(font, "VIEWPORT", Vector2::new(12f32, 12f32), 22f32, 1f32, Color::ORANGE);
//         // DRAW 3D BG
//         {
//             let mut d = d.begin_mode3D(*cam);
//             d.draw_grid(16i32, 1f32);
//         }
//         d.draw_text_ex(font, "Press Enter to start", Vector2::new(160f32, 500f32), 32f32, 1f32, Color::BLACK);
//     }
//     draw_on_target(&mut d, &render_target);
// }
