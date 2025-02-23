use raylib::prelude::*;

mod global;
mod greet_screen;
mod main_menu;

use crate::global::*;
use crate::greet_screen::*;
use crate::main_menu::*;

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
    rl.set_target_fps(MAX_FPS);

    let font: Font = rl
        .load_font_from_memory(&thread, ".ttf", MAIN_FONT, 64i32, None)
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

    // INIT MAIN MENU
    let mut main_menu: MainMenu = MainMenu::new();

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
                manage_main_menu(
                    &thread,
                    &mut rl,
                    &mut main_menu,
                    &delta_time,
                    &font,
                    &mut game_state,
                    &mut cam,
                    &mut render_target,
                );
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

/// Handle Greeting Screen State
/// When enter is pressed proceed to main menu and load saves
/// TODO: Remake into loading screen
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
    greet_screen.update(rl, delta_time, cam, game_state);
    greet_screen.draw(thread, rl, font, cam, render_target);
}

/// Handle Main Menu State
fn manage_main_menu(
    thread: &RaylibThread,
    rl: &mut RaylibHandle,
    main_menu: &mut MainMenu,
    delta_time: &f32,
    font: &Font,
    game_state: &mut GameState,
    cam: &mut Camera3D,
    render_target: &mut RenderTexture2D,
) {
    main_menu.update(rl, delta_time, cam, game_state);
    main_menu.draw(thread, rl, font, cam, render_target);
}

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
