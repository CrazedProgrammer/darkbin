extern crate process_path;
extern crate tiled;
extern crate sdl2;
extern crate time;
extern crate fnv;
#[macro_use]
mod util;
mod gfx;
mod game;

use game::Game;
use gfx::window::main_loop;

fn main() {
    let mut game = Game::new();
    main_loop(&mut game);
}
