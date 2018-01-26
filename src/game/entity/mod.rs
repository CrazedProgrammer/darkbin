use game::state::GameState;
use game::event::{Event,EntityAction};
use gfx::assets::Asset;
use sdl2::rect::Rect;
use util::Vec2;

pub mod player;

pub trait Entity {
    fn do_action(&mut self, action: &EntityAction, game: &GameState, shape: &mut EntityShape) -> Vec<Event>;
}

#[derive(Clone)]
pub struct EntityShape {
    id: u64,
    pub position: Vec2,
    pub size: Vec2,
    pub texture: Asset,
    pub texture_area: Option<Rect>,
}

impl EntityShape {
    pub fn new(id: u64) -> EntityShape {
        EntityShape {
            id: id,
            position: Vec2::new(0f32, 0f32),
            size: Vec2::new(0f32, 0f32),
            texture: Asset::None,
            texture_area: None,
        }
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
}
