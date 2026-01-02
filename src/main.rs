//! main.rs
//! Entry point for runa

use runa_tui::app;
use runa_tui::config::Config;
use runa_tui::core::terminal;
use runa_tui::utils::cli::{CliAction, handle_args};

fn main() -> std::io::Result<()> {
    match handle_args() {
        CliAction::Exit => return Ok(()),
        CliAction::RunApp => (),
    }

    let config = Config::load();
    let mut app = app::AppState::new(&config)?;
    terminal::run_terminal(&mut app)
}
