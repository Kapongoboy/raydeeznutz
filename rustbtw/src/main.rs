use raylib::prelude::*;

fn check_user_key(position: &mut Vector2, rl: &mut RaylibHandle) -> () {
        if rl.is_key_down(KeyboardKey::KEY_A) {
            position.x -= 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            position.x += 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            position.y += 2.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            position.y -= 2.0;
        }
}

fn check_user_mouse(
    position: &mut Vector2,
    rl: &mut RaylibHandle,
    ball_color: &mut Color
    ) -> () {
    *position = rl.get_mouse_position();

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_MIDDLE_BUTTON) {
        *ball_color = Color::MAROON;
    } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
        *ball_color = Color::LIME;
    } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        *ball_color = Color::PURPLE;
    }

}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 480)
        .title("Ordained v0.1.0")
        .build();

    let mut ball_position = Vector2::new(
        rl.get_screen_width() as f32 / 2.0,
        rl.get_screen_height() as f32 / 2.0,);

    let mut ball_color = Color::DARKPURPLE;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // check_user_key(&mut ball_position, &mut rl);
        check_user_mouse(&mut ball_position, &mut rl, &mut ball_color);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text(
            "Ordained the game for real this time",
            10, 10, 20, Color::DARKPURPLE);
        d.draw_circle_v(ball_position, 50.0, ball_color);
    }
}
