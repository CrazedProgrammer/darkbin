use game::entity::Entity;
use game::entity::player::Player;
use gfx::input::Input;
use gfx::assets::Assets;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::{Color};
use sdl2::rect::{Point, Rect};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

mod entity;
mod event;

pub struct GameState {
    pub d_time: f32,
    pub input: Input,
    entities: HashMap<u64, Rc<Entity>>,
    next_entity_id: u64,
}

pub struct Game {
    state: GameState,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            d_time: 1e-9f32,
            input: Input::new(),
            entities: HashMap::new(),
            next_entity_id: 0u64,
        }
    }

    pub fn add_entity(&mut self, entity: Rc<Entity>) -> u64 {
        let entity_id = self.next_entity_id;
        self.entities.insert(entity_id, entity);
        self.next_entity_id += 1u64;
        entity_id
    }
}

impl Clone for GameState {
    fn clone(&self) -> GameState {
        GameState {
            d_time: self.d_time,
            input: self.input.clone(),
            // is this the right indentation?
            entities: self.entities.iter()
                .map(|ref entity_pair|
                    (entity_pair.0.clone(), entity_pair.1.rc_clone()))
                .collect::<HashMap<_,_>>(),
            next_entity_id: self.next_entity_id,
        }
    }
}


impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
        }
    }

    pub fn init(&mut self) {
        self.state.add_entity(Rc::new(Player::new()));
    }

    pub fn update(&mut self, input: &Input, d_time: f32) {
        self.state.input = input.clone();
        self.state.d_time = d_time;
        let lock_state = self.state.clone();
        for mut entity_pair in self.state.entities.iter_mut() {
            let mut entity = Rc::get_mut(&mut entity_pair.1).unwrap();
            entity.update(&lock_state);
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        for entity_pair in self.state.entities.iter() {
            let entity = entity_pair.1;
            let shape = entity.get_shape();
            canvas.copy(&assets.get_texture(&shape.texture), None, Some(Rect::new(shape.position.0 as i32, shape.position.1 as i32, shape.size.0 as u32, shape.size.1 as u32))).unwrap();
        }
    }
}
