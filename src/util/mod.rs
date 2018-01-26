use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
use vek;

pub type Hasher = BuildHasherDefault<FnvHasher>;
pub type Vec2 = vek::Vec2<f32>;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
