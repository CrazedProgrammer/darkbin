use game::entity::Entity;
use game::entity::player::Player;
use gfx::input::Input;
use gfx::assets::Assets;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::{Color};
use std::rc::Rc;
use std::cell::RefCell;

mod entity;

#[derive(Clone)]
pub struct GameState {
    entities: Vec<Rc<Entity>>
}

pub struct Game {
}

impl Game {
    pub fn new() -> Game {
        Game {
            entities: vec![],
        }
    }

    pub fn init(&mut self) {
        self.entities.push(Some(Rc::new(Player::new())));
    }

    pub fn update(&mut self, input: &Input, d_time: f32) {
        for i in 0..self.entities.len() {
            let mut bor = self.entities[i].take().unwrap();
            let mut entity = Rc::get_mut(&mut bor).unwrap();
            entity.update(d_time, self);
            self.entities[i] = Some(Rc::new(entity));
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.copy(&assets.get_texture(&"test.png".to_string()), None, None).unwrap();
    }
}
