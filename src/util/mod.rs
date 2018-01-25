use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

pub type Hasher = BuildHasherDefault<FnvHasher>;

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
