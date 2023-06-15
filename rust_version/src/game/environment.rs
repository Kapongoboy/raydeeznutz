use raylib::prelude::*;

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

    pub fn draw(building: Building) {

    }
}

