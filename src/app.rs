use crate::config::Config;
use crate::file_manager::{FileEntry, read_dir};
use crate::formatter::Formatter;

pub struct AppState<'a> {
    pub entries: Vec<FileEntry>,
    pub selected: usize,
    pub config: &'a Config,
}

impl<'a> AppState<'a> {
    pub fn new(config: &'a Config) -> std::io::Result<Self> {
        let mut entries = read_dir(".")?;

        let formatter = Formatter::new(
            config.dirs_first,
            config.show_hidden,
            config.case_insensitive,
        );

        formatter.filter_hidden(&mut entries);

        Ok(Self {
            entries,
            selected: 0,
            config,
        })
    }

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
