use gfx::input::Input;
use gfx::viewport::Viewport;
use std::collections::HashMap;
use game::entity::EntityShape;
use util::Hasher;

pub struct GameState {
    pub input: Input,
    pub viewport: Viewport,
    pub shapes: HashMap<u64, EntityShape, Hasher>
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            input: Input::new(),
            viewport: Viewport::new(),
            shapes: HashMap::<_, _, _>::default(),
        }
    }
}
