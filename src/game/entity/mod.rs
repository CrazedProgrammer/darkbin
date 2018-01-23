use game::state::GameState;
use game::event::EntityAction;
use sdl2::rect::Point;
use std::rc::Rc;
use std::cell::RefCell;

pub mod player;

pub trait Entity {
    fn do_action(&mut self, action: &EntityAction, game: &GameState) -> Vec<EntityAction>;
    fn get_shape(&self) -> &EntityShape;
    fn rc_clone(&self) -> Rc<Entity>;
}

#[derive(Clone)]
pub struct EntityShape {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub texture: String,
}

impl EntityShape {
    fn new(position: (f32, f32), size: (f32, f32), texture: String) -> EntityShape {
        EntityShape {
            position: position,
            size: size,
            texture: texture,
        }
    }
}
