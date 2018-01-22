use gfx::input::Input;
use gfx::assets::Assets;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::{Color, PixelFormatEnum};

pub struct Game {
}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn init(&mut self) {
    }

    pub fn update(&mut self, input: &Input, d_time: f32) {
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.copy(&assets.get_texture(&"test.png".to_string()), None, None);
    }
}
