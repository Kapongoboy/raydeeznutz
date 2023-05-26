use raylib::prelude::*;
use KeyboardKey::*;

fn move_player(
    rl: &mut RaylibHandle, 
    player: &mut Rectangle,
    camera: &mut Camera2D
    ) -> () {
        if rl.is_key_down(KEY_A) {
            player.x -= 2.0;
        } else if rl.is_key_down(KEY_D) {
            player.x += 2.0;
        }

        camera.target.x = player.x + 20.0;
        camera.target.y =  player.y + 20.0;

        if rl.is_key_down(KEY_Q) {
            camera.rotation -= 1.0;
        } else if rl.is_key_down(KEY_E) {
            camera.rotation += 1.0;
        }

        if camera.rotation > 40.0 {
            camera.rotation = 40.0;
        } else if camera.rotation < - 40.0 {
            camera.rotation = -40.0;
        }

        camera.zoom += rl.get_mouse_wheel_move()*0.05;

        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        } else if camera.zoom < 0.1 {
            camera.zoom = 0.1;
        }

        if rl.is_key_pressed(KEY_R) {
            camera.zoom = 1.0;
            camera.rotation = 0.0;
        }
}

// fn check_user_key(position: &mut Vector2, rl: &mut RaylibHandle) -> () {
//         if rl.is_key_down(KeyboardKey::KEY_A) {
//             position.x -= 2.0;
//         }
//         if rl.is_key_down(KeyboardKey::KEY_D) {
//             position.x += 2.0;
//         }
//         if rl.is_key_down(KeyboardKey::KEY_S) {
//             position.y += 2.0;
//         }
//         if rl.is_key_down(KeyboardKey::KEY_W) {
//             position.y -= 2.0;
//         }
// }
//

// fn check_user_mouse(
//     position: &mut Vector2,
//     rl: &mut RaylibHandle,
//     ball_color: &mut Color
//     ) -> () {
//     *position = rl.get_mouse_position();
//
//     if rl.is_mouse_button_pressed(MouseButton::MOUSE_MIDDLE_BUTTON) {
//         *ball_color = Color::MAROON;
//     } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
//         *ball_color = Color::LIME;
//     } else if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
//         *ball_color = Color::PURPLE;
//     }
//
// }

fn main() {
    const MAX_BUILDINGS: usize = 100;
    const EMPTY_REC: Rectangle = Rectangle::new(0.0, 0.0, 0.0, 0.0);
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ordained v0.1.0")
        .build();

    // let ball_position = Vector2::new(
    //     SCREEN_WIDTH as f32 / 2.0,
    //     SCREEN_HEIGHT as f32 / 2.0,);

    // let ball_color = Color::DARKPURPLE;

    let mut player = Rectangle::new(400.0, 280.0, 40.0, 40.0);
    let mut buildings: [Rectangle; MAX_BUILDINGS] = [EMPTY_REC; MAX_BUILDINGS];
    let mut build_colors: [Color; MAX_BUILDINGS] = [Color::WHITE; MAX_BUILDINGS];

    let mut spacing = 0;
    for i in 0..buildings.len() {
        buildings[i].width = get_random_value::<i32>(50, 200) as f32;
        buildings[i].height = get_random_value::<i32>(100, 800) as f32;
        buildings[i].y = rl.get_screen_height() as f32 - 130.0 - buildings[i].height;
        buildings[i].x = -6000.0 + spacing as f32;

        spacing += buildings[i].width as i32;

        build_colors[i] = Color::new(
            get_random_value::<i32>(200, 240) as u8,
            get_random_value::<i32>(200, 240) as u8,
            get_random_value::<i32>(200, 240) as u8,
            128
            );
    }

    let mut camera = Camera2D::default();
    camera.target = Vector2::new(player.x + 20.0, player.y + 20.0);
    camera.rotation = 0.0;
    camera.zoom = 1.0;


    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // check_user_key(&mut ball_position, &mut rl);
        // check_user_mouse(&mut ball_position, &mut rl, &mut ball_color);
        //
        move_player(&mut rl, &mut player, &mut camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        d.begin_mode2D(camera);
        d.draw_rectangle(-6000, 320, 13000, 8000, Color::DARKGRAY);
        for i in 0..MAX_BUILDINGS {
            d.draw_rectangle_rec(buildings[i], build_colors[i]);
        }

        d.draw_rectangle_rec(player, Color::RED);

        d.draw_line(
            camera.target.x as i32,
            -SCREEN_WIDTH*10,
            camera.target.x as i32,
            SCREEN_HEIGHT*10,
            Color::GREEN
            );

        d.draw_line(
            -SCREEN_WIDTH*10,
            camera.target.y as i32,
            SCREEN_WIDTH*10,
            camera.target.y as i32,
            Color::GREEN
            );

        d.draw_text("SCREEN AREA", 640, 10, 20, Color::RED);

        d.draw_rectangle(0, 0, SCREEN_WIDTH, 5, Color::RED);
        d.draw_rectangle(0, 5, 5, SCREEN_HEIGHT, Color::RED);
        d.draw_rectangle(SCREEN_WIDTH - 5, 5, 5, SCREEN_HEIGHT - 10, Color::RED);
        d.draw_rectangle(0, SCREEN_HEIGHT - 5, SCREEN_WIDTH, 5, Color::RED);

        d.draw_rectangle(10, 10, 250, 113, Color::fade(&Color::SKYBLUE, 0.5));
        d.draw_rectangle_lines(10, 10, 250, 113, Color::BLUE);

        d.draw_text("Free 2d camera controls", 20, 20, 10, Color::BLACK);
        d.draw_text("- Right/Left to move offset", 40, 40, 10, Color::DARKGRAY);
        d.draw_text("- Mouse Wheel to Zoom in -out", 40, 60, 10, Color::DARKGRAY);
        d.draw_text("- A / S to Rotate", 40, 80, 10, Color::DARKGRAY);
        d.draw_text("- R to reset Zoom and Rotation", 40, 100, 10, Color::DARKGRAY);
    }
}
