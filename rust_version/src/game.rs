pub mod environment;
pub mod character;

use raylib::prelude::*;
use crate::config::Config;

const MAX_BUILDINGS: usize = 100;

pub struct Building {
    size: Vec<Rectangle>,
    colors: Vec<Color>,
    spacing: f32,
}

impl Building {
    pub fn build() -> Building {
        return Building {
            size: Vec::with_capacity(MAX_BUILDINGS),
            colors: Vec::with_capacity(MAX_BUILDINGS),
            spacing: 0.0,
        }
    }
}




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

