extern crate twox_hash;

use std::hash::BuildHasherDefault;
use twox_hash::XxHash;

pub type Hasher = BuildHasherDefault<XxHash>;
