//! Configuration options for runa
//!
//! This module defines all configuration options and deserializes them
//! from the runa.toml using serde.
//!
//! Each config struct corresponds to a top-level key in the runa.toml.

pub mod display;
pub mod input;
pub mod load;
pub mod theme;

pub use display::Display;
pub use input::{Editor, Keys};
pub use load::Config;
pub use theme::Theme;
