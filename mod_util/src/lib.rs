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
pub mod mod_loader;
pub mod mod_settings;
pub mod property_tree;

use mod_info::Version;
use mod_loader::Mod;
pub type UsedMods = HashMap<String, Mod>;
pub type UsedVersions = HashMap<String, Version>;
