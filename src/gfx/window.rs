extern crate sdl2;
extern crate time;

use game::Game;
use gfx::input::Input;
use gfx::assets::Assets;
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::rect::Point;
use sdl2::event::Event;

const ENABLE_VSYNC: bool = false;

pub fn main_loop(game: &mut Game) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem
        .window("Game of Powder", 800, 600)
        .position_centered()
        .resizable()
        .build().unwrap();

    let mut canvas;
    {
        let mut canvasbuilder = window.into_canvas();
        if ENABLE_VSYNC {
            canvasbuilder = canvasbuilder.present_vsync();
        }
        canvas = canvasbuilder.build().unwrap();
    }
    let mut texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut assets = Assets::new();
    assets.load_all(&mut texture_creator);
    game.init();

    let mut input = Input::new();
    let mut prev_nano_time = time::precise_time_ns();

    'event_loop: loop {
        input.push_next();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'event_loop; },

                Event::KeyDown {scancode: Some(scancode), ..} => {
                    input.key_down(scancode);
                },

                Event::KeyUp {scancode: Some(scancode), ..} => {
                    input.key_up(scancode);
                },

                Event::MouseButtonDown {mouse_btn, ..} => {
                    input.mouse_button_down(mouse_btn);
                },

                Event::MouseButtonUp {mouse_btn, ..} => {
                    input.mouse_button_up(mouse_btn);
                },

                Event::MouseMotion {x, y, ..} => {
                    input.mouse_pos = Point::new(x, y);
                },

                Event::MouseWheel {y, ..} => {
                    input.mouse_wheel = y;
                }

                _ => {},
            }
        }

        let cur_nano_time = time::precise_time_ns();
        let raw_d_time: f32 = (cur_nano_time - prev_nano_time) as f32 / 1e9f32 ;
        let d_time: f32 = if raw_d_time == 0f32 { 1e-9f32 } else { raw_d_time }; // to prevent divide by zero

        game.update(&input, d_time);
        prev_nano_time = cur_nano_time;

        game.draw(&mut canvas, &assets);
        canvas.present();
    }
}
