use game::entity::Entity;
use game::Game;
use sdl2::rect::Point;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Player {
    pos: Point,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Point::new(0, 0),
        }
    }
}

impl Entity for Player {
    fn update(&mut self, d_time: f32, game: &mut Game) {
        self.pos.x += 1;
    }

    fn rc_clone(&self) -> Rc<Entity> {
        Rc::new(self.clone())
    }
}
