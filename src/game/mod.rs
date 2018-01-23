use game::entity::Entity;
use game::entity::player::Player;
use gfx::input::Input;
use gfx::assets::Assets;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::{Color};
use sdl2::rect::{Point, Rect};
use game::event::{Event,Action,EntityAction};
use game::state::GameState;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

mod entity;
mod event;
mod state;

pub struct Game {
    state: GameState,
    events: Vec<Event>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
            events: vec![],
        }
    }

    pub fn init(&mut self) {
        self.events.push(Event::new(0f32, Action::AddEntity(Rc::new(Player::new(0f32)))));
    }

    pub fn update(&mut self, input: &Input, d_time: f32) {
        println!("Entities: {} FPS: {}", self.state.entities.len(), 1f32 / d_time);
        self.state.input = input.clone();
        let lock_state = self.state.clone();

        /*
        for entity_pair in self.state.entities.iter_mut() {
            self.events.push(Event::new(0f32, Action::DoEntity(*entity_pair.0, EntityAction::Update(d_time))));
        }
        */

        let mut i = 0;
        while i < self.events.len() {
            let mut event = (&self.events[i]).clone();
            event.time -= d_time;
            if event.time <= 0f32 {
                match event.action {
                    Action::AddEntity(entity) => {
                        self.state.add_entity(entity);
                    },
                    Action::DoEntity(entity_id, action) => {
                        match (self.state.entities.get_mut(&entity_id)) {
                            Some(mut entity_rc) => {
                                let entity = Rc::get_mut(entity_rc).unwrap();
                                let new_events = entity.do_action(&action, &lock_state);
                                self.events.extend(new_events);
                            },
                            None => { },
                        }
                    },
                    Action::RemoveEntity(entity_id) => {
                        self.state.remove_entity(entity_id);
                    },
                }
                self.events.remove(i);
            } else {
                self.events[i] = event;
                i += 1;
            }
        }

        for entity_rc in self.state.entities.iter_mut() {
            let mut entity = Rc::get_mut(entity_rc.1).unwrap();
            let new_events = entity.do_action(&EntityAction::Update(d_time), &lock_state);
            self.events.extend(new_events);
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
