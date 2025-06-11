//! GameVerse native function modules

pub mod streaming;
pub mod cam;
pub mod brain;
pub mod object;
pub mod vehicle;
pub mod audio;
pub mod graphics;
pub mod task;
pub mod player;
pub mod network;
pub mod ped;
pub mod misc;
pub mod entity;
pub mod weapon;
pub mod hud;

// Re-export all functions for convenience
pub use streaming::*;
pub use cam::*;
pub use brain::*;
pub use object::*;
pub use vehicle::*;
pub use audio::*;
pub use graphics::*;
pub use task::*;
pub use player::*;
pub use network::*;
pub use ped::*;
pub use misc::*;
pub use entity::*;
pub use weapon::*;
pub use hud::*;
