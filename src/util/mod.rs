use sdl2::rect::Rect;
use std::hash::BuildHasherDefault;
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

pub fn rect_part(id: u32, part_width: u32, part_height: u32, width: u32) -> Rect {
    let parts_per_line = width / part_width;
    let x = (id % parts_per_line) * part_width;
    let y = (id / parts_per_line) * part_height;
    Rect::new(x as i32, y as i32, part_width, part_height)
}
