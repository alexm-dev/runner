use crate::file_manager::{FileEntry, read_dir};
use crate::formatter::Formatter;

pub struct AppState {
    pub entries: Vec<FileEntry>,
    pub selected: usize,
}

impl AppState {
    pub fn new() -> std::io::Result<Self> {
        let mut entries = read_dir(".")?;

        let formatter = Formatter::new(true, false, false);

        formatter.filter_hidden(&mut entries); // filters + sorts

        Ok(Self {
            entries,
            selected: 0,
        })
    }
}
