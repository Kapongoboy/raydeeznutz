use raylib::prelude::*;
const MAX_BUILDINGS: usize = 100;

pub mod game;

fn main() {
    use raylib::consts::KeyboardKey::*;
    let (w, h) = (800, 450);
    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("raydeezrust")
        .build();

    game::run(&mut rl, &thread);
}



