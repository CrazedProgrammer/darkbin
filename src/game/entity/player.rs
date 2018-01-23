use game::entity::{Entity, EntityShape};
use game::event::{Event, EntityAction, Action};
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
    pub fn new(x: f32) -> Player {
        Player {
            shape: EntityShape::new((x, 0f32), (32f32, 32f32), "test.png".to_string()),
        }
    }
}

impl Entity for Player {
    fn do_action(&mut self, action: &EntityAction, state: &GameState) -> Vec<Event> {
        let mut actions: Vec<Event> = vec![];
        match action {
            &EntityAction::Update(d_time) => {
                if state.input.get_key(Scancode::Space) {
                    self.shape.position.0 += 60f32 * d_time;
                    if state.input.get_key_down(Scancode::Space) {
                        actions.push(Event::new(0f32, Action::AddEntity(Rc::new(Player::new(self.shape.position.0 + 10f32)))));
                    }
                }
            },
            _ => { },
        }
        actions
    }

    fn get_shape(&self) -> &EntityShape {
        &self.shape
    }

    fn rc_clone(&self) -> Rc<Entity> {
        Rc::new(self.clone())
    }
}
