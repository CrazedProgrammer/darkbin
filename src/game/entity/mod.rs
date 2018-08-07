use game::state::GameState;
use game::event::{Event,EntityAction};
use gfx::assets::Asset;
use sdl2::rect::Rect;
use util::Vec2;

pub mod player;
pub mod camera;
pub mod particle;
pub mod e_crate;

pub trait Entity {
    fn do_action(&mut self, action: &EntityAction, game: &GameState, shape: &mut EntityShape) -> Vec<Event>;
}

#[derive(Clone)]
pub struct EntityShape {
    id: u64,
    pub entity_type: EntityType,
    pub position: Vec2,
    pub size: Vec2,
    pub origin: Option<Vec2>,
    pub angle: f32,
    pub texture: Asset,
    pub texture_area: Option<Rect>,
    pub hitbox: Vec<EntityBox>,
}

#[derive(Clone)]
pub struct EntityBox {
    pub position: Vec2,
    pub size: f32,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum EntityType {
    None,
    Camera,
    Player,
    Particle,
    Crate,
}

impl EntityShape {
    pub fn new(id: u64) -> EntityShape {
        EntityShape {
            id: id,
            entity_type: EntityType::None,
            position: Vec2::new(0f32, 0f32),
            size: Vec2::new(0f32, 0f32),
            origin: None,
            angle: 0f32,
            texture: Asset::None,
            texture_area: None,
            hitbox: vec![],
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn collides_with(&self, other: &EntityShape) -> bool {
        for hbox in self.hitbox.iter() {
            for other_hbox in other.hitbox.iter() {
                if hbox.relativise(&self).collides_with(&other_hbox.relativise(other)) {
                    return true;
                }
            }
        }
        false
    }
}

impl EntityBox {
    pub fn new(position: Vec2, size: f32) -> EntityBox {
        EntityBox {
            position: position,
            size: size,
        }
    }

    pub fn relativise(&self, shape: &EntityShape) -> EntityBox {
        EntityBox {
            position: shape.position + self.position.rotated_z(shape.angle),
            size: self.size
        }
    }

    pub fn collides_with(&self, other: &EntityBox) -> bool {
        self.position.distance(other.position) < (self.size + other.size) / 2f32
    }
}

