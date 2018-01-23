use gfx::input::Input;
use gfx::viewport::Viewport;
use std::collections::HashMap;
use std::rc::Rc;
use game::entity::Entity;

pub struct GameState {
    pub input: Input,
    pub viewport: Viewport,
    pub entities: HashMap<u64, Rc<Entity>>,
    next_entity_id: u64,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            input: Input::new(),
            viewport: Viewport::new(),
            entities: HashMap::new(),
            next_entity_id: 0u64,
        }
    }

    pub fn add_entity(&mut self, entity: Rc<Entity>) -> u64 {
        let entity_id = self.next_entity_id;
        self.entities.insert(entity_id, entity);
        self.next_entity_id += 1u64;
        entity_id
    }

    pub fn remove_entity(&mut self, entity_id: u64) {
        self.entities.remove(&entity_id);
    }
}

impl Clone for GameState {
    fn clone(&self) -> GameState {
        GameState {
            input: self.input.clone(),
            viewport: self.viewport.clone(),
            // is this the right indentation?
            entities: self.entities.iter()
                .map(|ref entity_pair|
                    (entity_pair.0.clone(), entity_pair.1.rc_clone()))
                .collect::<HashMap<_,_>>(),
            next_entity_id: self.next_entity_id,
        }
    }
}
