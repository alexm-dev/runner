mod app;
mod cli;
mod config;
mod file_manager;
mod formatter;
mod keymap;
mod terminal;
mod utils;
mod worker;

use cli::{CliAction, handle_args};
use config::Config;

fn main() -> std::io::Result<()> {
    match handle_args() {
        CliAction::Exit => return Ok(()),
        CliAction::RunApp => (),
    }

    let config = Config::load();
    let mut app = app::AppState::new(&config)?;
    terminal::run_terminal(&mut app)
}
