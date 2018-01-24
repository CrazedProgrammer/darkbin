extern crate sdl2;
extern crate time;
extern crate fnv;
mod gfx;
mod game;
mod util;

use game::Game;
use gfx::window::main_loop;

fn main() {
    let mut game = Game::new();
    main_loop(&mut game);
}
