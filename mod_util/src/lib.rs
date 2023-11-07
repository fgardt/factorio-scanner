#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(dead_code)]

use std::collections::HashMap;

mod any_basic;
pub use any_basic::*;

pub mod mod_info;
pub mod mod_list;
pub mod mod_settings;
pub mod property_tree;

pub type UsedMods = HashMap<String, mod_info::Version>;
