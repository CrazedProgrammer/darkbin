use game::state::GameState;
use game::event::{Event,EntityAction};
use gfx::assets::Asset;

pub mod player;

pub trait Entity {
    fn do_action(&mut self, action: &EntityAction, game: &GameState, shape: &mut EntityShape) -> Vec<Event>;
}

#[derive(Clone)]
pub struct EntityShape {
    id: u64,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub texture: Asset,
}

impl EntityShape {
    pub fn new(id: u64, position: (f32, f32), size: (f32, f32), texture: Asset) -> EntityShape {
        EntityShape {
            id: id,
            position: position,
            size: size,
            texture: texture,
        }
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
}
