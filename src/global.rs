use raylib::prelude::*;

// CONSTANTS
pub const MAX_FPS: u32 = 60;
pub const SCREEN_HEIGHT: i32 = 640;
pub const SCREEN_WIDTH: i32 = 480;
pub const MAIN_FONT: &[u8; 46020] = include_bytes!("../fonts/Catholicon.ttf");

// GAMESTATES
pub enum GameState {
    GreetingScreen,   // Press enter
    MainMenu,         // Menu start quit settings, etc
    Playing,          // GameLoop goes
        Pause,            // Gameloop poused
        GameOver,         // Player lost all lifes
        EndScreen,        // Player won and titles are shown
}

/// DRAW TEXTURE TARGET
/// Draws viewport saving its dpi
pub fn draw_on_target(d: &mut RaylibDrawHandle, render_target: &RenderTexture2D) {
    // Screen scaling
    let mut scaling: f32 = 1f32;

    let scale_x: f32;
    let scale_y: f32;

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
