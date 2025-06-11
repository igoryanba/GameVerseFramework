//! GameVerse Native Function Wrappers
//!
//! This crate provides type-safe Rust wrappers for all GameVerse native functions.
//! Generated from FiveM documentation with enhanced type safety and error handling.
//!
//! # Example
//!
//! ```rust
//! use gameverse_natives::prelude::*;
//!
//! fn example() -> NativeResult<()> {
//!     let player_id = PlayerId(0);
//!     let ped = get_player_ped_safe(player_id)?;
//!     let position = get_entity_coords_safe(ped.into())?;
//!     println!("Player position: {:?}", position);
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod errors;
pub mod natives;

// Re-export commonly used items
pub mod prelude {
    pub use crate::types::*;
    pub use crate::errors::{NativeError, NativeResult};
    pub use crate::natives::*;
}

pub use prelude::*;
