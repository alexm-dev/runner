use crate::file_manager::FileEntry;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub struct NavState {
    current_dir: PathBuf,
    entries: Vec<FileEntry>,
    selected: usize,
    positions: HashMap<PathBuf, usize>,
    request_id: u64,
}

impl NavState {
    pub fn new(path: PathBuf) -> Self {
        Self {
            current_dir: path,
            entries: Vec::new(),
            selected: 0,
            positions: HashMap::new(),
            request_id: 0,
        }
    }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }
    pub fn entries(&self) -> &[FileEntry] {
        &self.entries
    }
    pub fn selected_idx(&self) -> usize {
        self.selected
    }
    pub fn request_id(&self) -> u64 {
        self.request_id
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected)
    }

    pub fn prepare_new_request(&mut self) -> u64 {
        self.request_id += 1;
        self.request_id
    }

    pub fn move_up(&mut self) -> bool {
        let len = self.entries.len();
        if len == 0 {
            return false;
        }

        if self.selected == 0 {
            self.selected = len - 1;
        } else {
            self.selected -= 1;
        }
        true
    }

    pub fn move_down(&mut self) -> bool {
        let len = self.entries.len();
        if len == 0 {
            return false;
        }

        self.selected = (self.selected + 1) % len;
        true
    }

    pub fn save_position(&mut self) {
        self.positions
            .insert(self.current_dir.clone(), self.selected);
    }

    pub fn get_position(&self) -> &HashMap<PathBuf, usize> {
        &self.positions
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.current_dir = path;
        self.entries.clear();
        self.selected = 0;
        // instantly kills all pending messages from the previous directory.
        self.request_id += 1;
    }

    pub fn update_from_worker(
        &mut self,
        path: PathBuf,
        entries: Vec<FileEntry>,
        focus: Option<OsString>,
    ) {
        self.current_dir = path;
        self.entries = entries;

        if let Some(f) = focus {
            self.selected = self
                .entries
                .iter()
                .position(|e| e.name() == &f)
                .unwrap_or(0);
        } else {
            self.selected = self.positions.get(&self.current_dir).cloned().unwrap_or(0);
        }

        // Safety check with saturating_sub to avoid panic on empty dirs
        self.selected = self.selected.min(self.entries.len().saturating_sub(1));
    }
}
