use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

pub type Hasher = BuildHasherDefault<FnvHasher>;
