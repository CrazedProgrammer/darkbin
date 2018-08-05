extern crate sdl2;

use game::Game;
use gfx::input::Input;
use gfx::assets::Assets;
use sdl2::image::{INIT_PNG, INIT_JPG};
use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::event::{Event,WindowEvent};
use util::Vec2;
use util::perf::{Perf, PerfType};

const ENABLE_VSYNC: bool = true;
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub fn main_loop(game: &mut Game) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let window = video_subsystem
        .window("Game of Powder", WINDOW_WIDTH, WINDOW_HEIGHT)
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
    input.window_size = Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
    let mut perf = Perf::new();

    'event_loop: loop {
        perf.start_interval(PerfType::Total);

        println!("Game time: {:.2} ms  FPS: {:.2}", 1000f64 * (perf.prev_interval(PerfType::Total) - perf.prev_interval(PerfType::VSync)), 1f64 / perf.average_interval_sec(PerfType::Total));

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
                    input.mouse_pos = Vec2::new(x as f32, y as f32);
                },

                Event::MouseWheel {y, ..} => {
                    input.mouse_wheel = y;
                },

                Event::Window {win_event, ..} => {
                    match win_event {
                        WindowEvent::SizeChanged(width, height) => {
                            input.window_size = Vec2::new(width as f32, height as f32);
                        },
                        _ => { },
                    }
                }

                _ => {},
            }
        }


        let raw_d_time: f32 = perf.prev_interval(PerfType::Total) as f32;
        let d_time: f32 = if raw_d_time == 0f32 { 1e-9f32 } else { raw_d_time }; // to prevent divide by zero

        perf.start_interval(PerfType::Update);
        game.update(&input, d_time);
        perf.end_interval(PerfType::Update);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        perf.start_interval(PerfType::VSync);
        canvas.clear();
        perf.end_interval(PerfType::VSync);

        perf.start_interval(PerfType::Draw);
        game.draw(&mut canvas, &assets);
        perf.end_interval(PerfType::Draw);
        canvas.present();

        perf.end_interval(PerfType::Total);
    }
}
