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

    rl.set_window_min_size(240i32, 320i32);
    rl.set_target_fps(60u32);

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

    while !rl.window_should_close() {
        // PRE-UPDATE, GLOBAL KEYBOARD INPUT, ETC. | Probably will not be needed
        let delta_time: f32 = rl.get_frame_time();

        // STATE MANAGING
        match game_state {
            // include upd + draw to each state since they have different logic
            // and drawing tasks
            GameState::GreetingScreen => {
                greet(&thread, &mut rl, delta_time, &mut game_state, &mut cam, &mut render_target);
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

/// Drawing a centered text
fn draw_text_center(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    text: &str,
    y: i32,
    font_size: i32,
    color: Color,
) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(
        text,
        // SCREEN_WIDTH is a constant, so if screen is resizeable, it is better
        // to use d.get_screen_width();
        (SCREEN_WIDTH as i32 / 2i32) - (text_length / 2),
        y,
        font_size,
        color,
    );
}


/// Handle Greeting Screen State
/// When enter is pressed proceed to main menu and load saves
fn greet(
    thread: &RaylibThread,
    rl: &mut RaylibHandle,
    delta_time: f32,
    game_state: &mut GameState,
    cam: &mut Camera3D,
    render_target: &mut RenderTexture2D
) {
    // Update
    {
        // exammple update
        rl.update_camera(cam, CameraMode::CAMERA_ORBITAL);
        
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER){
            *game_state = GameState::MainMenu;
        }
    }

    // DRAW OUT OF VIEWPORT
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("WHOLE WINDOW", 12, 12, 20, Color::ORANGE);

        // DRAW IN VIEWPORT
        {
            let mut d = d.begin_texture_mode(&thread, render_target);
            d.clear_background(Color::WHITE);
            d.draw_text("VIEWPORT", 12, 12, 20, Color::ORANGE);
            // DRAW 3D BG
            {
                let mut d = d.begin_mode3D(*cam);
                d.draw_grid(16i32, 1f32);
            }
            draw_text_center(&mut d, "Press Enter to start", 500i32, 32i32, Color::BLACK);
        }
        draw_on_target(&mut d, &render_target);
    }
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
