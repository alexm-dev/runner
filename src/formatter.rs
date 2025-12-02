use crate::file_manager::FileEntry;

pub struct Formatter {
    pub dirs_first: bool,
    pub show_hidden: bool,
}

impl Formatter {
    pub fn new(dirs_first: bool, show_hidden: bool) -> Self {
        Self {
            dirs_first,
            show_hidden,
        }
    }

    pub fn format(&self, entries: &mut [FileEntry]) {
        if self.dirs_first {
            entries.sort_unstable_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });
        } else {
            entries.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        }
    }
    pub fn filter_hidden(&self, entries: &mut Vec<FileEntry>) {
        if !self.show_hidden {
            entries.retain(|e| !e.is_hidden);
        }
        self.format(entries);
    }
} // impl Formatter
