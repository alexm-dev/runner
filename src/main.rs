use runner_tui::app;
use runner_tui::cli::{CliAction, handle_args};
use runner_tui::config::Config;
use runner_tui::terminal;

fn main() -> std::io::Result<()> {
    match handle_args() {
        CliAction::Exit => return Ok(()),
        CliAction::RunApp => (),
    }

    let config = Config::load();
    let mut app = app::AppState::new(&config)?;
    terminal::run_terminal(&mut app)
}
