mod app;
mod file_manager;
mod formatter;
mod terminal;

// use crate::file_manager::FileEntry;
// use crate::formatter::Formatter;

fn main() -> std::io::Result<()> {
    let mut app = app::AppState::new()?;
    terminal::run_terminal()
}
