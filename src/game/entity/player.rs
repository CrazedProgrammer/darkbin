use game::entity::{Entity, EntityShape};
use game::event::EntityAction;
use game::state::GameState;
use sdl2::rect::Point;
use sdl2::keyboard::Scancode;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Player {
    shape: EntityShape,
}

impl Player {
    pub fn new() -> Player {
        Player {
            shape: EntityShape::new((0f32, 0f32), (32f32, 32f32), "test.png".to_string()),
        }
    }
}

impl Entity for Player {
    fn do_action(&mut self, action: &EntityAction, state: &GameState) -> Vec<EntityAction> {
        match action {
            &EntityAction::Update(d_time) => {
                if state.input.get_key(Scancode::Space) {
                    self.shape.position.0 += 60f32 * d_time;
                }
            }
            _ => { }
        }
        vec![]
    }

    fn get_shape(&self) -> &EntityShape {
        &self.shape
    }

    fn rc_clone(&self) -> Rc<Entity> {
        Rc::new(self.clone())
    }
}
