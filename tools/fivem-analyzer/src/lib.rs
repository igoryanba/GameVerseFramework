//! Static FiveM resource inventory. Resource Lua is never executed.
pub mod scanner;
pub use scanner::{analyze, to_gameverse_toml, CompatibilityCategory, Framework, ResourceReport};
