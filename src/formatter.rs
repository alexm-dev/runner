use crate::file_manager::FileEntry;

pub struct Formatter {
    pub dirs_first: bool,
    pub show_hidden: bool,
    pub case_insensitive: bool,
}

impl Formatter {
    pub fn new(dirs_first: bool, show_hidden: bool, case_insensitive: bool) -> Self {
        Self {
            dirs_first,
            show_hidden,
            case_insensitive,
        }
    }

    pub fn format(&self, entries: &mut [FileEntry]) {
        let cmp_name = |a: &FileEntry, b: &FileEntry| {
            if self.case_insensitive {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            } else {
                a.name.cmp(&b.name)
            }
        };
        if self.dirs_first {
            entries.sort_unstable_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => cmp_name(a, b),
            });
        } else {
            entries.sort_unstable_by(cmp_name);
        }
    }
    pub fn filter_hidden(&self, entries: &mut Vec<FileEntry>) {
        if !self.show_hidden {
            entries.retain(|e| !e.is_hidden);
        }
        self.format(entries);
    }
} // impl Formatter
