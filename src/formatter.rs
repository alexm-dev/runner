use crate::file_manager::FileEntry;
use std::collections::HashSet;
use std::ffi::OsString;
use std::sync::Arc;

pub struct Formatter {
    dirs_first: bool,
    show_hidden: bool,
    show_system: bool,
    case_insensitive: bool,
    always_show: Arc<HashSet<OsString>>,
    always_show_lowercase: Arc<HashSet<String>>,
}

impl Formatter {
    pub fn new(
        dirs_first: bool,
        show_hidden: bool,
        show_system: bool,
        case_insensitive: bool,
        always_show: Arc<HashSet<OsString>>,
    ) -> Self {
        let always_show_lowercase = Arc::new(
            always_show
                .iter()
                .map(|s| s.to_string_lossy().to_lowercase())
                .collect::<HashSet<String>>(),
        );
        Self {
            dirs_first,
            show_hidden,
            show_system,
            case_insensitive,
            always_show,
            always_show_lowercase,
        }
    }

    pub fn format(&self, entries: &mut [FileEntry]) {
        entries.sort_by(|a, b| {
            if self.dirs_first {
                match (a.is_dir(), b.is_dir()) {
                    (true, false) => return std::cmp::Ordering::Less,
                    (false, true) => return std::cmp::Ordering::Greater,
                    _ => {}
                }
            }

            if self.case_insensitive {
                a.lowercase_name().cmp(b.lowercase_name())
            } else {
                a.name().cmp(b.name())
            }
        });
    }

    pub fn filter_entries(&self, entries: &mut Vec<FileEntry>) {
        entries.retain(|e| {
            let is_exception = if self.case_insensitive {
                self.always_show_lowercase.contains(e.lowercase_name())
            } else {
                self.always_show.contains(e.name())
            };

            if is_exception {
                return true;
            }

            let hidden_ok = self.show_hidden || !e.is_hidden();
            let system_ok = self.show_system || !e.is_system();
            hidden_ok && system_ok
        });
        self.format(entries);
    }
} // impl Formatter
