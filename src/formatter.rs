use crate::file_manager::FileEntry;

pub struct Formatter {
    pub dirs_first: bool,
}

impl Formatter {
    pub fn new(dirs_first: bool) -> Self {
        Self { dirs_first }
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
}
