pub mod perf;

use sdl2::rect::Rect;
use std::hash::BuildHasherDefault;
use std::f32;
use fnv::FnvHasher;
use vek;

pub type Hasher = BuildHasherDefault<FnvHasher>;
pub type Vec2 = vek::Vec2<f32>;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::<_,_,Hasher>::default();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[inline]
pub fn rect_part(id: u32, part_width: u32, part_height: u32, width: u32) -> Rect {
    let parts_per_line = width / part_width;
    let x = (id % parts_per_line) * part_width;
    let y = (id / parts_per_line) * part_height;
    Rect::new(x as i32, y as i32, part_width, part_height)
}

#[inline]
pub fn optional_origin(origin: Option<Vec2>, size: Vec2) -> Vec2 {
    match origin {
        Some(origin) => origin,
        None => size / 2f32,
    }
}

#[inline]
pub fn angle_to(from: Vec2, to: Vec2) -> f32 {
    let difference: Vec2 = to - from;
    difference.y.atan2(difference.x)
}

#[inline]
pub fn to_angle(angle: f32, distance: f32) -> Vec2 {
    Vec2::new(angle.cos() * distance, angle.sin() * distance)
}
