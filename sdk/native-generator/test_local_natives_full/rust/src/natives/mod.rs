//! GameVerse native function modules

pub mod vehicle;
pub mod player;
pub mod ped;

// Re-export all functions for convenience
pub use vehicle::*;
pub use player::*;
pub use ped::*;
