use game::entity::{Entity, EntityShape, EntityBox, EntityType};
use game::event::{Event, EntityAction, Action};
use game::state::GameState;
use gfx::assets::Asset;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use std::rc::Rc;
use util::{Vec2,angle_to,to_angle};


#[derive(Clone)]
pub struct Crate {
    start_position: Vec2,
}

impl Crate {
    pub fn new(position: Vec2) -> Crate {
        Crate {
            start_position: position,
        }
    }
}

impl Entity for Crate {
    fn do_action(&mut self, action: &EntityAction, state: &GameState, shape: &mut EntityShape) -> Vec<Event> {
        let mut actions: Vec<Event> = vec![];
        match action {
            &EntityAction::Init => {
                shape.entity_type = EntityType::Crate;
                shape.position = self.start_position;
                shape.size = Vec2::new(32f32, 32f32);
                shape.texture = Asset::Crate;
                shape.hitbox.push(EntityBox::new(Vec2::zero(), 32f32));
            },
            &EntityAction::Update(d_time) => {
            },
            _ => { },
        }
        actions
    }
}
