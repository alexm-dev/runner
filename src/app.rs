use crate::config::Config;
use crate::file_manager::FileEntry;
use crate::keymap::{Action, Keymap};
use crate::utils::open_in_editor;
use crate::worker::{WorkerResponse, WorkerTask, start_worker};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};

pub enum KeypressResult {
    Continue,
    Quit,
    OpenedEditor,
}

/// Application state of the file browser
///
/// Stores the directory entires, the current selection, the directory positions
/// and the configuration as a reference.
pub struct AppState<'a> {
    current_dir: std::path::PathBuf,
    entries: Vec<FileEntry>,
    selected: usize,
    config: &'a Config,
    dir_positions: HashMap<std::path::PathBuf, usize>,
    keymap: Keymap,
    worker_tx: Sender<WorkerTask>,
    response_rx: Receiver<WorkerResponse>,

    pub is_loading: bool,
    pub preview_content: Vec<String>,
    pub current_preview_path: Option<PathBuf>,
    pub parent_content: Vec<String>,
}

impl<'a> AppState<'a> {
    pub fn new(config: &'a Config) -> std::io::Result<Self> {
        let (worker_tx, task_rx) = mpsc::channel();
        let (res_tx, response_rx) = mpsc::channel();
        let current_dir = std::env::current_dir()?;

        // Start the engine
        start_worker(task_rx, res_tx);

        let mut app = Self {
            current_dir,
            entries: Vec::new(),
            selected: 0,
            config,
            dir_positions: HashMap::new(),
            keymap: Keymap::from_config(config),
            worker_tx,
            response_rx,
            is_loading: false,
            preview_content: vec!["Loading...".into()],
            current_preview_path: None,
            parent_content: vec!["Loading...".into()],
        };

        app.request_dir_load(None);
        Ok(app)
    }

    fn request_dir_load(&mut self, focus: Option<std::ffi::OsString>) {
        self.is_loading = true;
        let _ = self.worker_tx.send(WorkerTask::LoadDirectory {
            path: self.current_dir.clone(),
            focus,
            dirs_first: self.config.dirs_first(),
            show_hidden: self.config.show_hidden(),
            show_system: self.config.show_system(),
            case_insensitive: self.config.case_insensitive(),
            always_show: self.config.always_show().to_vec(),
        });
    }

    pub fn tick(&mut self) -> bool {
        let mut changed = false;
        while let Ok(response) = self.response_rx.try_recv() {
            changed = true;
            match response {
                WorkerResponse::DirectoryLoaded {
                    path,
                    entries,
                    focus,
                } => {
                    if path == self.current_dir {
                        self.entries = entries;
                        self.is_loading = false;

                        self.selected = if let Some(target) = focus {
                            self.entries
                                .iter()
                                .position(|e| e.name() == &target)
                                .unwrap_or(0)
                        } else {
                            self.dir_positions
                                .get(&self.current_dir)
                                .cloned()
                                .unwrap_or(0)
                        };
                        self.selected = self.selected.min(self.entries.len().saturating_sub(1));
                        self.request_preview();
                        self.request_parent_conent();
                    } else if Some(path.as_path()) == self.current_dir.parent() {
                        self.parent_content = entries
                            .iter()
                            .map(|e| {
                                let name = e.name().to_string_lossy();
                                if e.is_dir() {
                                    format!("{}/", name)
                                } else {
                                    name.into_owned()
                                }
                            })
                            .collect()
                    }
                }
                WorkerResponse::PreviewLoaded { path, lines } => {
                    if Some(path) == self.current_preview_path {
                        self.preview_content = lines;
                    }
                }
                WorkerResponse::Error(e) => {
                    self.preview_content = vec![e];
                }
            }
        }
        changed
    }

    fn request_preview(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            let path = self.current_dir.join(entry.name());
            if self.current_preview_path.as_ref() != Some(&path) {
                self.current_preview_path = Some(path.clone());
                let _ = self.worker_tx.send(WorkerTask::LoadPreview {
                    path,
                    max_lines: 60,
                });
            }
        }
    }
    fn request_parent_conent(&mut self) {
        if let Some(parent_path) = self.current_dir.parent() {
            let _ = self.worker_tx.send(WorkerTask::LoadDirectory {
                path: parent_path.to_path_buf(),
                focus: None,
                dirs_first: self.config.dirs_first(),
                show_hidden: self.config.show_hidden(),
                show_system: self.config.show_system(),
                case_insensitive: self.config.case_insensitive(),
                always_show: self.config.always_show().to_vec(),
            });
        } else {
            self.parent_content = vec!["/".into()];
        }
    }

    pub fn current_dir(&self) -> &std::path::Path {
        &self.current_dir
    }

    pub fn visible_entries(&self) -> &[FileEntry] {
        &self.entries
    }

    pub fn has_visible_entries(&self) -> bool {
        !self.entries.is_empty()
    }

    pub fn visible_selected(&self) -> Option<usize> {
        if self.entries.is_empty() {
            None
        } else {
            Some(self.selected)
        }
    }

    pub fn config(&self) -> &Config {
        self.config
    }

    fn save_current_pos(&mut self) {
        self.dir_positions
            .insert(self.current_dir.clone(), self.selected);
    }

    pub fn handle_keypress(&mut self, key: &str) -> KeypressResult {
        match self.keymap.lookup(key) {
            Some(Action::GoParent) => self.handle_go_parent(),
            Some(Action::GoIntoDir) => self.handle_go_into_dir(),
            Some(Action::GoUp) => self.handle_go_up(),
            Some(Action::GoDown) => self.handle_go_down(),
            Some(Action::Open) => self.handle_open_file(),
            Some(Action::Quit) => self.handle_quit(),
            _ => KeypressResult::Continue,
        }
    }

    fn handle_go_parent(&mut self) -> KeypressResult {
        if let Some(parent) = self.current_dir.parent() {
            let parent_path = parent.to_path_buf();
            let exited_dir_name = self.current_dir.file_name().map(|n| n.to_os_string());
            self.save_current_pos();
            self.current_dir = parent_path;
            self.request_dir_load(exited_dir_name);
        }
        KeypressResult::Continue
    }

    fn handle_go_into_dir(&mut self) -> KeypressResult {
        if let Some(entry) = self.entries.get(self.selected)
            && entry.is_dir()
        {
            let dir_name = entry.name().clone();
            self.save_current_pos();
            self.current_dir = self.current_dir.join(&dir_name);
            self.entries.clear();
            self.selected = 0;
            self.request_dir_load(None);
        }
        KeypressResult::Continue
    }

    fn handle_go_up(&mut self) -> KeypressResult {
        if self.selected > 0 {
            self.selected -= 1;
            self.request_preview();
        }
        KeypressResult::Continue
    }

    fn handle_go_down(&mut self) -> KeypressResult {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
            self.request_preview();
        }
        KeypressResult::Continue
    }

    fn handle_open_file(&mut self) -> KeypressResult {
        if let Some(entry) = self.entries.get(self.selected) {
            let path = self.current_dir.join(entry.name());
            if let Err(e) = open_in_editor(self.config.editor(), &path) {
                eprintln!("Error opening editor: {}", e);
            }
            return KeypressResult::OpenedEditor;
        }
        KeypressResult::Continue
    }

    fn handle_quit(&self) -> KeypressResult {
        KeypressResult::Quit
    }
}
