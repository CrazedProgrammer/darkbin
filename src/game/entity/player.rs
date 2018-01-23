use game::entity::{Entity, EntityShape};
use game::GameState;
use sdl2::rect::Point;
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
    fn update(&mut self, state: &GameState) {
        self.shape.position.0 += 60f32 * state.d_time;
        println!("{}", state.d_time);
    }

    fn get_shape(&self) -> &EntityShape {
        &self.shape
    }

    fn rc_clone(&self) -> Rc<Entity> {
        Rc::new(self.clone())
    }
}
