// Less spamming during builds in early development
#![allow(dead_code)]
#[allow(unused_imports)]
use fake65c02_sys::*;

mod io;
mod state;
mod memory_layout;

pub use memory_layout::{MemoryBank, MemoryLayout};
pub use state::State;
