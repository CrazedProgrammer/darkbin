use game::entity::{Entity, EntityShape};
use game::event::{Event, EntityAction, Action};
use game::state::GameState;
use sdl2::keyboard::Scancode;
use std::rc::Rc;

#[derive(Clone)]
pub struct Player {
    x: f32,
}

impl Player {
    pub fn new(x: f32) -> Player {
        Player {
            x: x,
        }
    }
}

impl Entity for Player {
    fn do_action(&mut self, action: &EntityAction, state: &GameState, shape: &mut EntityShape) -> Vec<Event> {
        let mut actions: Vec<Event> = vec![];
        match action {
            &EntityAction::Init => {
                shape.position = (self.x, 0f32);
                shape.size = (32f32, 32f32);
                shape.texture = "test.png".to_string();
            },
            &EntityAction::Update(d_time) => {
                if state.input.get_key(Scancode::Space) {
                    shape.position.0 += 60f32 * d_time;
                    if state.input.get_key_down(Scancode::Space) {
                        actions.push(Event::new(0f32, Action::AddEntity(Rc::new(Player::new(shape.position.0 + 10f32)))));
                    }
                }
            },
        }
        actions
    }
}
