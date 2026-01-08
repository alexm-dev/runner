//! Internal library crate for runa.
//!
//! The shipped application is the `rn` binary (`src/main.rs`).
//!
//! This library exists to share code between targets (binary, tests) and to keep modules organized.

pub mod app;
pub mod config;
pub mod core;
pub mod ui;
pub mod utils;
