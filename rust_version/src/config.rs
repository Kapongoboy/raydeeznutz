pub struct Config {
    pub width: i32,
    pub height: i32,
    pub title: String,
}

impl Config {
pub fn new(w: i32, h: i32, title: String) -> Config {
    Config {
        width: w,
        height: h,
        title
    }
}
}
