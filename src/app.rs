use crate::config::Config;
use crate::file_manager::{FileEntry, browse_dir};
use crate::formatter::Formatter;

/// Application state of the file browser
///
/// Stores the directory entires, the current selection
/// and the configuration as a reference.
pub struct AppState<'a> {
    pub current_dir: std::path::PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected: usize,
    pub config: &'a Config,
}

impl<'a> AppState<'a> {
    pub fn new(config: &'a Config) -> std::io::Result<Self> {
        let current_dir = std::env::current_dir()?;
        let mut entries = browse_dir(&current_dir)?;

        let formatter = Formatter::new(
            config.dirs_first,
            config.show_hidden,
            config.case_insensitive,
        );

        formatter.filter_hidden(&mut entries);

        Ok(Self {
            current_dir,
            entries,
            selected: 0,
            config,
        })
    }

    /// Handles a keypress
    ///
    /// Returns false if the application should exit
    /// TODO: Implement more keypress defaults.
    pub fn handle_keypress(&mut self, key: &str) -> bool {
        if self.config.keys.go_up.iter().any(|k| k == key) {
            if self.selected > 0 {
                self.selected -= 1;
            }
            return true;
        }
        if self.config.keys.go_down.iter().any(|k| k == key) {
            if self.selected + 1 < self.entries.len() {
                self.selected += 1;
            }
            return true;
        }
        if self.config.keys.quit.iter().any(|k| k == key) {
            return false;
        }
        true
    }
}
