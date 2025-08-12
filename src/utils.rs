use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Instant};
use rand::Rng;

pub const COLOR_GRAY: Color = Color::RGBA(100, 100, 100, 255);
pub const COLOR_DARK_GRAY: Color = Color::RGBA(30, 30, 30, 255);
pub const COLOR_LIGHT_GRAY: Color = Color::RGBA(200, 200, 200, 255);
pub const COLOR_RED: Color = Color::RGBA(255, 0, 0, 255);
pub const COLOR_BLUE: Color = Color::RGBA(0, 0, 255, 255);

pub fn set_background_color(canvas: &mut Canvas<Window>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
}

pub struct MillisTimer {
    last: Instant,
}

impl MillisTimer {
    pub fn new_now() -> Self { Self { last: Instant::now() } }

    pub fn elapsed_ms(&self) -> u128 {
        self.last.elapsed().as_millis()
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
    }
}

pub fn rand_inclusive(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}


