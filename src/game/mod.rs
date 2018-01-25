use game::entity::{Entity,EntityShape};
use game::entity::player::Player;
use gfx::input::Input;
use gfx::assets::Assets;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use game::event::{Event,Action,EntityAction};
use game::state::GameState;
use std::rc::Rc;
use std::collections::HashMap;
use util::Hasher;

mod entity;
mod event;
mod state;

// for performance debugging purposes
const DISABLE_DRAW: bool = true;

pub struct Game {
    state: GameState,
    entities: HashMap<u64, Rc<Entity>, Hasher>,
    next_entity_id: u64,
    events: Vec<Event>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
            entities: HashMap::<_, _, _>::default(),
            next_entity_id: 0u64,
            events: vec![],
        }
    }

    pub fn init(&mut self) {
        self.events.push(Event::new(0f32, Action::AddEntity(Rc::new(Player::new(0f32)))));
        self.state.viewport.zoom = 10f32;
    }

    pub fn update(&mut self, input: &Input, d_time: f32) {
        println!("Entities: {} FPS: {}", self.entities.len(), 1f32 / d_time);
        self.state.input = input.clone();

        for event in self.events.iter_mut() {
            event.time -= d_time;
        }

        let mut i = 0;
        while i < self.events.len() {
            let event = (&self.events[i]).clone();
            if event.time <= 0f32 {
                match event.action {
                    Action::AddEntity(entity) => {
                        self.add_entity(entity);
                    },
                    Action::DoEntity(entity_id, action) => {
                        self.do_entity(entity_id, &action);
                    },
                    Action::RemoveEntity(entity_id) => {
                        self.remove_entity(entity_id);
                    },
                }
                self.events.remove(i);
            } else {
                self.events[i] = event;
                i += 1;
            }
        }

        let entity_ids: Vec<u64> = self.entities.keys().map(|x| x.clone()).collect();
        for entity_id in entity_ids.iter() {
            self.do_entity(*entity_id, &EntityAction::Update(d_time));
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        if DISABLE_DRAW {
            return;
        }

        for shape_pair in self.state.shapes.iter() {
            let shape = shape_pair.1;
            let window_size = canvas.window().size();
            let center_x: f32 = (shape.position.0 - self.state.viewport.position.0) * self.state.viewport.zoom + (window_size.0 as f32) / 2f32;
            let center_y: f32 = (shape.position.1 - self.state.viewport.position.1) * self.state.viewport.zoom + (window_size.1 as f32) / 2f32;
            let width: f32 = shape.size.0 * self.state.viewport.zoom;
            let height: f32 = shape.size.1 * self.state.viewport.zoom;
            let x: i32 = (center_x - width / 2f32) as i32;
            let y: i32 = (center_y - height / 2f32) as i32;
            canvas.copy(&assets.get_texture(&shape.texture), None, Some(Rect::new(x, y, width as u32, height as u32))).unwrap();
        }
    }

    fn add_entity(&mut self, entity: Rc<Entity>) -> u64 {
        let entity_id = self.next_entity_id;
        self.entities.insert(entity_id, entity);
        self.state.shapes.insert(entity_id, EntityShape::new(entity_id, (0f32, 0f32), (0f32, 0f32), "none".to_string()));
        self.events.push(Event::new(0f32, Action::DoEntity(entity_id, EntityAction::Init)));
        self.next_entity_id += 1u64;
        entity_id
    }

    fn do_entity(&mut self, entity_id: u64, action: &EntityAction) {
        match self.entities.get_mut(&entity_id) {
            Some(entity_rc) => {
                let mut entity = Rc::get_mut(entity_rc).unwrap();
                let mut shape = self.state.shapes.get(&entity_id).unwrap().clone();
                let new_events = entity.do_action(&action, &self.state, &mut shape);
                self.events.extend(new_events);
                self.state.shapes.insert(entity_id, shape);
            },
            None => { },
        }
    }

    fn remove_entity(&mut self, entity_id: u64) {
        self.entities.remove(&entity_id);
        self.state.shapes.remove(&entity_id);
    }
}
