use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use vek::ops::Clamp;
use std::f32;
use std::f32::consts;
use std::rc::Rc;
use std::collections::HashMap;
use std::mem;
use util::{Hasher,Vec2,optional_origin};
use gfx::input::Input;
use gfx::assets::Assets;
use game::entity::{Entity,EntityShape};
use game::entity::player::Player;
use game::entity::camera::Camera;
use game::entity::e_crate::Crate;
use game::event::{Event,Action,EntityAction};
use game::state::GameState;

mod entity;
mod event;
mod state;

// for performance debugging purposes
const DISABLE_DRAW: bool = false;

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
        self.events.push(Event::new(0f32, Action::AddEntity(Rc::new(Camera::new()))));
        self.events.push(Event::new(0f32, Action::AddEntity(Rc::new(Player::new(0f32)))));
        self.events.push(Event::new(0f32, Action::AddEntity(Rc::new(Crate::new(Vec2::new(100f32, 100f32))))));
        self.state.viewport.zoom = 2f32;
    }

    pub fn update(&mut self, input: &Input, d_time: f32) {
        print!("Entities: {}  ", self.entities.len());
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
                    Action::ChangeViewport(viewport) => {
                        self.state.viewport = viewport;
                    },
                }
                self.events.remove(i);
            } else {
                self.events[i] = event;
                i += 1;
            }
        }

        // check collisions
        let mut collisions: Vec<(u64, u64)> = vec![];
        for shape in self.state.shapes.values() {
            for other_shape in self.state.shapes.values() {
                if shape.get_id() != other_shape.get_id() {
                    if shape.collides_with(other_shape) {
                        collisions.push((shape.get_id(), other_shape.get_id()));
                    }
                }
            }
        }
        for (mut shape_id, mut other_shape_id) in collisions.iter_mut() {
            if other_shape_id > shape_id {
                mem::swap(&mut shape_id, &mut other_shape_id);
            }
        }
        collisions.iter().map(|x| println!("{} {}", x.0, x.1));
        collisions.sort_unstable();
        collisions.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
        for (shape_id, other_shape_id) in collisions.iter() {
            self.do_entity(*shape_id, &EntityAction::CollideWith(*other_shape_id));
            //self.do_entity(*other_shape_id, &EntityAction::CollideWith(*shape_id));
            // TODO: what causes duplicates????
        }

        // update event every frame
        let entity_ids: Vec<u64> = self.entities.keys().map(|x| x.clone()).collect();
        for entity_id in entity_ids.iter() {
            self.do_entity(*entity_id, &EntityAction::Update(d_time));
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) {
        if DISABLE_DRAW {
            return;
        }

        // Draw the map
        {
            let map = assets.get_map(&self.state.current_map);
            // NOTE: tile size has to be the same for all layers and tilesets
            let tile_width = map.tilesets[0].tile_width;
            let tile_height = map.tilesets[0].tile_height;
            let horizontal_tiles = map.layers[0].tiles[0].len();
            let vertical_tiles = map.layers[0].tiles.len();
            let tiles_rect = self.screen_to_tile_rect(tile_width, tile_height, horizontal_tiles, vertical_tiles);
            if tiles_rect.w > 0 && tiles_rect.h > 0 {
                for layer in map.layers.iter() {
                    for y in (tiles_rect.y)..(tiles_rect.y + tiles_rect.h) {
                        for x in (tiles_rect.x)..(tiles_rect.x + tiles_rect.w) {
                            let gid = layer.tiles[y as usize][x as usize];
                            if gid == 0u32 {
                                continue;
                            }
                            let tile_attrs = assets.get_map_gid(&self.state.current_map, gid);
                            let tile_size = Vec2::new(tile_width as f32, tile_height as f32);
                            let tile_position = Vec2::new(x as f32, y as f32) * tile_size;
                            let tile_screen_rect = self.rect_to_screen(tile_position, tile_size);
                            canvas.copy(&tile_attrs.0, Some(tile_attrs.1), Some(tile_screen_rect)).unwrap();
                        }
                    }
                }
            }
        }
        // Draw the entities
        for shape in self.state.shapes.values() {
            let origin = optional_origin(shape.origin, shape.size);
            let screen_origin = origin * self.state.viewport.zoom;
            let rect = self.rect_to_screen(shape.position - origin, shape.size);

            canvas.copy_ex(&assets.get_texture(&shape.texture), shape.texture_area, Some(rect), shape.angle.to_degrees() as f64, Point::new(screen_origin.x as i32, screen_origin.y as i32), false, false).unwrap();
        }
        // Draw the hitboxes
        for shape in self.state.shapes.values() {
            for hbox in shape.hitbox.iter() {
                let rhbox = hbox.relativise(shape);
                let sizev2 = Vec2::new(rhbox.size, rhbox.size);
                let rect = self.rect_to_screen(rhbox.position - sizev2 / 2f32, sizev2);
                canvas.draw_rect(Rect::new(rect.x as i32, rect.y as i32, rect.w as u32, rect.h as u32));
            }
        }
    }

    fn add_entity(&mut self, entity: Rc<Entity>) -> u64 {
        let entity_id = self.next_entity_id;
        self.entities.insert(entity_id, entity);
        self.state.shapes.insert(entity_id, EntityShape::new(entity_id));
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

    #[inline]
    fn position_to_screen(&self, position: Vec2) -> Point {
        let pos = (position - self.state.viewport.position) * self.state.viewport.zoom + self.state.input.window_size / 2f32;
        Point::new(pos.x as i32, pos.y as i32)
    }

    #[inline]
    fn rect_to_screen(&self, position: Vec2, size: Vec2) -> Rect {
        let lefttop_point = self.position_to_screen(position);
        let rightbottom = position + size;
        let rightbottom_point = self.position_to_screen(rightbottom);
        let size_point = rightbottom_point - lefttop_point;
        Rect::new(lefttop_point.x, lefttop_point.y, size_point.x as u32, size_point.y as u32)
    }

    fn screen_to_tile_rect(&self, tile_width: u32, tile_height: u32, horizontal_tiles: usize, vertical_tiles: usize) -> Rect {
        let tiles = Vec2::new(horizontal_tiles as f32, vertical_tiles as f32);
        let tile_size = Vec2::new(tile_width as f32, tile_height as f32);
        let lefttop_tile = (self.state.viewport.position - (self.state.input.window_size / self.state.viewport.zoom) / 2f32) / tile_size;
        let number_of_tiles = (self.state.input.window_size / self.state.viewport.zoom) / tile_size + 2f32;

        let lefttop_tile_clamped = lefttop_tile.clamped(Vec2::zero(), tiles - 1f32);
        let mut number_of_tiles_clamped = number_of_tiles;
        // if the left-top tile is either to the left or above the map, clamp the number of tiles
        if lefttop_tile.x < 0f32 || lefttop_tile.y < 0f32 {
            number_of_tiles_clamped = number_of_tiles_clamped.clamped(Vec2::zero(), (number_of_tiles + lefttop_tile - 1f32).clamped(Vec2::zero(), Vec2::new(f32::INFINITY, f32::INFINITY)));
        }
        // clamp number of tiles for when the bottom-right tile is to the right or bottom of the map
        number_of_tiles_clamped = number_of_tiles_clamped.clamped(Vec2::new(0f32, 0f32), tiles - lefttop_tile_clamped.floor());

        Rect::new(lefttop_tile_clamped.x as i32, lefttop_tile_clamped.y as i32, number_of_tiles_clamped.x as u32, number_of_tiles_clamped.y as u32)
    }
}
