use game::entity::{Entity, EntityShape, EntityType};
use game::event::{Event, EntityAction, Action};
use game::state::GameState;
use gfx::assets::Asset;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use std::rc::Rc;
use util::{Vec2,angle_to,to_angle};


#[derive(Clone)]
pub struct Particle {
    time_left: f32,
    position: Vec2,
    angle: f32,
    asset: Asset,
    size: Vec2,
    origin: Option<Vec2>,
    direction: f32,
    speed: f32,
}

impl Particle {
    pub fn new(position: Vec2, angle: f32, asset: Asset, size: Vec2, origin: Option<Vec2>, time_left: f32, direction: f32, speed: f32) -> Particle {
        Particle {
            time_left: time_left,
            position: position,
            angle: angle,
            asset: asset,
            size: size,
            origin: origin,
            direction: direction,
            speed: speed,
        }
    }

    pub fn new_static(position: Vec2, angle: f32, asset: Asset, size: Vec2, origin: Option<Vec2>, time_left: f32) -> Particle {
        Particle {
            time_left: time_left,
            position: position,
            angle: angle,
            asset: asset,
            size: size,
            origin: origin,
            direction: 0f32,
            speed: 0f32,
        }
    }
}

impl Entity for Particle {
    fn do_action(&mut self, action: &EntityAction, state: &GameState, shape: &mut EntityShape) -> Vec<Event> {
        let mut actions: Vec<Event> = vec![];
        match action {
            &EntityAction::Init => {
                shape.entity_type = EntityType::Particle;
                shape.position = self.position;
                shape.size = self.size;
                shape.texture = self.asset.clone();
                shape.origin = self.origin;
                shape.angle = self.angle;
            },
            &EntityAction::Update(d_time) => {
                shape.position += to_angle(self.direction, self.speed) * d_time;
                self.speed -= 500f32 * d_time;
                if self.speed < 0f32 {
                    self.speed = 0f32;
                }
                self.time_left -= d_time;
                if self.time_left <= 0f32 {
                    actions.push(Event::new(0f32, Action::RemoveEntity(shape.get_id())));
                }
            },
        }
        actions
    }
}
