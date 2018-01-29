use game::entity::{Entity, EntityShape, EntityType};
use game::event::{Event, EntityAction, Action};
use game::state::GameState;
use gfx::assets::Asset;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use std::rc::Rc;
use util::Vec2;

const PLAYER_SPEED: f32 = 200f32;

#[derive(Clone)]
pub struct Player {
    x: f32,
}

impl Player {
    pub fn new(x: f32) -> Player {
        Player {
            x: x,
        }
    }
}

impl Entity for Player {
    fn do_action(&mut self, action: &EntityAction, state: &GameState, shape: &mut EntityShape) -> Vec<Event> {
        let mut actions: Vec<Event> = vec![];
        match action {
            &EntityAction::Init => {
                shape.entity_type = EntityType::Player;
                shape.position = Vec2::new(self.x, 0f32);
                shape.size = Vec2::new(32f32, 24f32);
                shape.texture = Asset::PlayerP250;
                shape.texture_area = Some(Rect::new(0, 0, 32, 24));
                shape.origin = Some(Vec2::new(10f32, 12f32));
            },
            &EntityAction::Update(d_time) => {
                if state.input.get_key(Scancode::W) {
                    shape.position.y -= PLAYER_SPEED * d_time;
                }
                if state.input.get_key(Scancode::A) {
                    shape.position.x -= PLAYER_SPEED * d_time;
                }
                if state.input.get_key(Scancode::S) {
                    shape.position.y += PLAYER_SPEED * d_time;
                }
                if state.input.get_key(Scancode::D) {
                    shape.position.x += PLAYER_SPEED * d_time;
                }
                if state.input.get_key_down(Scancode::Space) {
                    actions.push(Event::new(0f32, Action::AddEntity(Rc::new(Player::new(shape.position.x + 10f32)))));
                }
            },
        }
        actions
    }
}
