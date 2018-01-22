use game::Game;
use std::rc::Rc;
use std::cell::RefCell;

pub mod player;

pub trait Entity {
    fn update(&mut self, d_time: f32, game: &mut Game);
    fn rc_clone(&self) -> Rc<Entity>;
}
