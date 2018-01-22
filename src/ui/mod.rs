extern crate sdl2;
extern crate time;

mod input;

use ui::input::Input;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use sdl2::rect::{Point, Rect};
use sdl2::event::{Event, WindowEvent};
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::keyboard::Keycode;

pub fn main_loop() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game of Powder", 800, 600)
        .position_centered()
        .resizable()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut input = Input::new();

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'event_loop; },
                _ => {},
            }
        }
    }
}
