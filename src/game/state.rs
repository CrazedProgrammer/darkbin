use gfx::input::Input;
use gfx::viewport::Viewport;
use gfx::assets::Asset;
use std::collections::HashMap;
use game::entity::{EntityShape, EntityType};
use util::Hasher;

pub struct GameState {
    pub input: Input,
    pub viewport: Viewport,
    pub shapes: HashMap<u64, EntityShape, Hasher>,
    pub current_map: Asset,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            input: Input::new(),
            viewport: Viewport::new(),
            shapes: HashMap::<_, _, _>::default(),
            current_map: Asset::TestMap,
        }
    }

    pub fn search_first_type(&self, entity_type: EntityType) -> Option<u64> {
        for (id, shape) in self.shapes.iter() {
            if shape.entity_type == entity_type {
                return Some(*id);
            }
        }
        None
    }
}
