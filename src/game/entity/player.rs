use game::entity::{Entity, EntityShape, EntityBox, EntityType};
use game::entity::particle::Particle;
use game::event::{Event, EntityAction, Action};
use game::state::GameState;
use gfx::assets::Asset;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use std::rc::Rc;
use std::f32::consts;
use util::{Vec2,angle_to,to_angle};

const PLAYER_SPEED: f32 = 200f32;

#[derive(Clone)]
pub struct Player {
    x: f32,
    shoot_left: f32,
}

impl Player {
    pub fn new(x: f32) -> Player {
        Player {
            x: x,
            shoot_left: 0f32,
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
                shape.hitbox.push(EntityBox::new(Vec2::new(12f32, 0f32), 24f32));
                shape.hitbox.push(EntityBox::new(Vec2::new(-20f32, 0f32), 12f32));
            },
            &EntityAction::Update(d_time) => {
                // TODO: refactor this
                let mut movement = Vec2::zero();
                if state.input.get_key(Scancode::W) {
                    movement.y -= PLAYER_SPEED;
                }
                if state.input.get_key(Scancode::A) {
                    movement.x -= PLAYER_SPEED;
                }
                if state.input.get_key(Scancode::S) {
                    movement.y += PLAYER_SPEED;
                }
                if state.input.get_key(Scancode::D) {
                    movement.x += PLAYER_SPEED;
                }
                let moved = movement != Vec2::zero();
                shape.position += movement * d_time;
                if state.input.get_key_down(Scancode::Space) {
                    actions.push(Event::new(0f32, Action::AddEntity(Rc::new(Player::new(shape.position.x + 10f32)))));
                }
                self.shoot_left -= d_time;
                if state.input.get_mouse_button_down(MouseButton::Left) && self.shoot_left <= 0f32 {
                    actions.push(Event::new(0f32, Action::AddEntity(Rc::new(Particle::new(shape.position + to_angle(shape.angle, 22f32), shape.angle, Asset::MuzzleFlash, Vec2::new(12f32, 8f32), Some(Vec2::new(1f32, 3f32)), 0.05f32, angle_to(Vec2::zero(), movement), if moved { PLAYER_SPEED } else { 0f32 })))));
                    actions.push(Event::new(0f32, Action::AddEntity(Rc::new(Particle::new(shape.position + to_angle(shape.angle, 16f32), shape.angle, Asset::EmptyShell, Vec2::new(4f32, 2f32), Some(Vec2::new(2f32, 1f32)), 100f32, shape.angle - consts::PI / 2f32, 100f32)))));
                    self.shoot_left = 0.15f32;
                }

                shape.angle = angle_to(state.input.window_size / 2f32, state.input.mouse_pos);
                shape.texture_area = Some(Rect::new(if self.shoot_left > 0f32 { 32 } else { 0 }, 0, 32, 24));
            },
            &EntityAction::CollideWith(other_id) => {
                println!("aa im colliding {} {}", shape.get_id(), other_id);
            },
        }
        actions
    }
}
