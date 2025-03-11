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

    // Minimum window size
    rl.set_window_min_size(240i32, 320i32);

    // Initial window size
    rl.set_window_size(320, 320);
    rl.set_window_position(
        rl.get_screen_width() / 2 - 160,
        rl.get_screen_height() / 2 - 160,
    );

    // INIT GAME DATA
    let mut gd: GameData = GameData::new();
    gd.load_config(&mut rl);
    // TODO: make a gd.init function, that will do everything needed on init

    // Setting max fps
    rl.set_target_fps(gd.get_max_fps());

    // LOAD FONT FROM MEMORY
    let font: Font = rl
        .load_font_from_memory(&thread, ".ttf", MAIN_FONT, 84i32, None)
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

    // INIT AUDIO DEVICE
    let audio = RaylibAudio::init_audio_device().unwrap();
    //audio.set_audio_stream_buffer_size_default(4096i32);
    unsafe {
        ffi::SetAudioStreamBufferSizeDefault(4096i32);
    }

    //Music::set_volume(&mut self, volume);
    //Sound::set_volume(&mut self, volume);

    // INIT GREETING SCREEN
    let mut greet_screen: GreetScreen = GreetScreen::new();

    // INIT MAIN MENU
    let mut main_menu: MainMenu = MainMenu::new();

    while !rl.window_should_close() && !gd.window_should_close() {
        // PRE-UPDATE, GLOBAL KEYBOARD INPUT, ETC. | Probably will not be needed
        let delta_time: f32 = rl.get_frame_time();

        // UPDATE
        match game_state {
            // include upd + draw to each state since they have different logic
            // and drawing tasks
            GameState::GreetingScreen => {
                greet_screen.update(&rl, &delta_time, &mut cam, &mut game_state);
            }
            GameState::MainMenu => {
                main_menu.update(&mut rl, &mut gd, &delta_time, &mut cam, &mut game_state);
            }
            GameState::Playing => {
                // play
            }
            GameState::GameOver => {
                // GameOver
            }
            GameState::EndScreen => {
                // EndScreen
            }
        }

        // DRAW out of canvas
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // DRAW IN CANVAS
        // STATE MANAGING
        match game_state {
            // include upd + draw to each state since they have different logic
            // and drawing tasks
            GameState::GreetingScreen => {
                greet_screen.draw(&thread, &mut d, &font, &cam, &mut render_target);
            }
            GameState::MainMenu => {
                main_menu.draw(&thread, &mut d, &gd, &font, &cam, &mut render_target);
            }
            GameState::Playing => {
                //playing
            }
            GameState::GameOver => {
                //game_over
            }
            GameState::EndScreen => {
                //end_screen
            }
        }

        // Draw FPS if global setting tells so
        //d.draw_fps(0, 30);
        if gd.fps_should_draw() {
            let current_fps: &String = &format!("{} fps", d.get_fps());
            d.draw_text_ex(
                &font,
                current_fps,
                Vector2::new(3f32, 3f32),
                32f32,
                1f32,
                Color::BLACK,
            );
            d.draw_text_ex(
                &font,
                current_fps,
                Vector2::new(2f32, 2f32),
                32f32,
                1f32,
                Color::WHITE,
            );
        }
    }
}

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
