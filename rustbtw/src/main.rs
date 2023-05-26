use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 480)
        .title("Ordained v0.1.0")
        .build();

    let mut ball_position = Vector2::new(
        rl.get_screen_width() as f32 / 2.0,
        rl.get_screen_height() as f32 / 2.0,);

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_A) {
            ball_position.x -= 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            ball_position.x += 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            ball_position.y += 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            ball_position.y -= 2.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text(
            "Ordained the game for real this time",
            10, 10, 20, Color::DARKPURPLE);
        d.draw_circle_v(ball_position, 50.0, Color::MAROON);
    }
}
