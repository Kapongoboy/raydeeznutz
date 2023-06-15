pub mod environment;
pub mod character;

use raylib::prelude::*;
use crate::config::Config;




pub fn run(config: Config) {
    let (mut rl, thread) = raylib::init()
        .size(config.width, config.height)
        .title(&config.title)
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
    }
}

