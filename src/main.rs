use raylib::prelude::*;

// Constants

//  consider 320x240 or 960x720
//  if 960x720 rewrite the scaling function, so it works well with lower res
//  screens and screns that are not 2 times bigger, meaning scale properly, or
//  add by +-320x240 or just strictly resize
//  BlueRevolver implemented Multiple scaling methods
const SCREEN_HEIGHT: i32 = 320;
const SCREEN_WIDTH: i32 = 240;

// TODO: Think through this enum, and decide what should be here?
enum GameState {
    GreetingScreen,
    MainMenu,
    ChooseDifficulty,
    Playing,
    Pause,
}

fn main() {
    // INIT WINDOW
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .title("NMF")
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
        
        // PRE-UPDATE
        {
            // ...
        }

        // UPDATE GAME
        {
            // ...
        }

        // DRAW WHOLE WINDOW
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            d.draw_text("WHOLE WINDOW", 12, 12, 20, Color::WHITE);

            // DRAW GAME VIEWPORT
            {
                let mut d = d.begin_texture_mode(&thread, &mut render_target);
                d.clear_background(Color::WHITE);
                d.draw_text("VIEWPORT", 12, 12, 20, Color::BLACK);
                
                // DRAW 3D
                {
                    let mut d = d.begin_mode3D(cam_background_3d);
                    d.draw_grid(16i32, 1f32);
                }
            }
            
            // DRAW TEXTURE TARGET
            {
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
        }
    }
}
