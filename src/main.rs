use raylib::prelude::*;

// CONSTANTS
const SCREEN_HEIGHT: i32 = 640;
const SCREEN_WIDTH: i32 = 480;

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

    rl.set_window_min_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    rl.set_target_fps(60u32);

    // INIT CAMERA
    let cam_background_3d = Camera3D::perspective(
        Vector3::new(0f32, 10f32, 10f32),
        Vector3::new(0f32, 0f32, 0f32),
        Vector3::new(0f32, 1f32, 0f32),
        45f32,
    );

    // INIT RENDER TARGET
    let mut render_target: RenderTexture2D = rl
        .load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .unwrap();

    while !rl.window_should_close() {
        // PRE-UPDATE, GLOBAL KEYBOARD INPUT, ETC. | Probably will not be needed
        {
            // ...
        }

        // UPDATE GAME
        {
            // ...
        }

        // STATE MANAGING
        match game_state {
            // include upd + draw to each state since they have different logic
            // and drawing tasks
            GameState::GreetingScreen => {
                greet();
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

        // DRAW OUT OF VIEWPORT
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            d.draw_text("WHOLE WINDOW", 12, 12, 20, Color::WHITE);

            // DRAW IN VIEWPORT
            {
                let mut d = d.begin_texture_mode(&thread, &mut render_target);
                d.clear_background(Color::WHITE);
                d.draw_text("VIEWPORT", 12, 12, 20, Color::BLACK);

                // DRAW 3D BG
                {
                    let mut d = d.begin_mode3D(cam_background_3d);
                    d.draw_grid(16i32, 1f32);
                }
            }
            draw_on_target(&mut d, &render_target);
        }
    }
}

/// DRAW TEXTURE TARGET
/// Draws An image in perfect resolution
/// TODO: make it imperfect
fn draw_on_target(d: &mut RaylibDrawHandle, render_target: &RenderTexture2D) {
    // Screen scaling
    let mut scaling: i32 = 1i32;

    let scale_y = d.get_screen_height() / SCREEN_HEIGHT as i32;
    let scale_x = d.get_screen_width() / SCREEN_WIDTH as i32;

    if scale_x != scaling && scale_y != scaling {
        if render_target.texture.width * scaling <= d.get_screen_width()
            && render_target.texture.height * scaling <= d.get_screen_height()
        {
            if scale_x >= scale_y {
                scaling = scale_y as i32;
            } else {
                scaling = scale_x as i32;
            }
        }
    }

    let screen_center: Vector2 = Vector2::new(
        d.get_screen_width() as f32 / 2f32,
        d.get_screen_height() as f32 / 2f32,
    );

    let render_target_position: Vector2 = Vector2::new(
        screen_center.x - (render_target.texture.width * scaling) as f32 / 2f32,
        screen_center.y - (render_target.texture.height * scaling) as f32 / 2f32,
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
            render_target.texture.width * scaling,
            render_target.texture.height * scaling,
        ),
        rvec2(-render_target_position.x, -render_target_position.y),
        0f32,
        Color::WHITE,
    );
}

/// Handle Greeting Screen State
/// When enter is pressed proceed to main menu and load saves
fn greet() {}

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
