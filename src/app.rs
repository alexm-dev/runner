mod nav;
mod parent;
mod preview;

pub use nav::NavState;
pub use parent::ParentState;
pub use preview::{PreviewData, PreviewState};

use crate::config::Config;
use crate::file_manager::FileEntry;
use crate::keymap::{Action, Keymap};
use crate::utils::open_in_editor;
use crate::worker::{WorkerResponse, WorkerTask, start_worker};
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::sync::Arc;

pub enum KeypressResult {
    Continue,
    Quit,
    OpenedEditor,
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutMetrics {
    pub parent_width: usize,
    pub main_width: usize,
    pub preview_width: usize,
    pub preview_height: usize,
}

impl Default for LayoutMetrics {
    fn default() -> Self {
        Self {
            parent_width: 20,
            main_width: 40,
            preview_width: 40,
            preview_height: 50,
        }
    }
}

pub struct AppState<'a> {
    config: &'a Config,
    keymap: Keymap,

    pub metrics: LayoutMetrics,

    pub nav: NavState,
    pub preview: PreviewState,
    pub parent: ParentState,

    worker_tx: Sender<WorkerTask>,
    response_rx: Receiver<WorkerResponse>,
    pub is_loading: bool,
}

impl<'a> AppState<'a> {
    pub fn new(config: &'a Config) -> std::io::Result<Self> {
        let (worker_tx, task_rx) = unbounded::<WorkerTask>();
        let (res_tx, response_rx) = unbounded::<WorkerResponse>();
        let current_dir = std::env::current_dir()?;

        start_worker(task_rx, res_tx);

        let mut app = Self {
            config,
            keymap: Keymap::from_config(config),
            metrics: LayoutMetrics::default(),
            nav: NavState::new(current_dir),
            preview: PreviewState::new(),
            parent: ParentState::new(),
            worker_tx,
            response_rx,
            is_loading: false,
        };

        app.request_dir_load(None);
        app.request_parent_content();
        Ok(app)
    }

    pub fn config(&self) -> &Config {
        self.config
    }

    pub fn visible_entries(&self) -> &[FileEntry] {
        self.nav.entries()
    }
    pub fn visible_selected(&self) -> Option<usize> {
        if self.nav.entries().is_empty() {
            None
        } else {
            Some(self.nav.selected_idx())
        }
    }
    pub fn has_visible_entries(&self) -> bool {
        !self.nav.entries().is_empty()
    }

    /// The heart of the app: updates state and handles worker messages
    pub fn tick(&mut self) -> bool {
        let mut changed = false;

        // Handle preview debounc
        if self.preview.should_trigger() {
            self.request_preview();
            changed = true;
        }

        // Process worker response
        while let Ok(response) = self.response_rx.try_recv() {
            changed = true;
            match response {
                WorkerResponse::DirectoryLoaded {
                    path,
                    entries,
                    focus,
                    request_id,
                } => {
                    // only update nav if BOTH the ID and path match.
                    if request_id == self.nav.request_id() && path == self.nav.current_dir() {
                        self.nav.update_from_worker(path, entries, focus);
                        self.is_loading = false;
                        self.request_preview();
                        self.request_parent_content();
                    }
                    // PREVIEW CHECK: Must match the current preview request
                    else if request_id == self.preview.request_id() {
                        self.preview.update_from_entries(entries, request_id);
                        if let Some(entry) = self.nav.selected_entry() {
                            let path = self.nav.current_dir().join(entry.name());
                            if let Some(&cached_pos) = self.nav.get_position().get(&path) {
                                self.preview.set_selected_idx(cached_pos);
                            } else {
                                self.preview.set_selected_idx(0);
                            }
                        }
                    }
                    // PARENT CHECK: Must match the current parent request
                    else if request_id == self.parent.request_id() {
                        let current_name = self
                            .nav
                            .current_dir()
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();

                        self.parent
                            .update_from_entries(entries, &current_name, request_id);
                    }
                }
                WorkerResponse::PreviewLoaded { lines, request_id } => {
                    self.preview.update_content(lines, request_id);
                }
                WorkerResponse::Error(e) => {
                    self.preview.set_error(e);
                }
            }
        }
        changed
    }

    //  Action handlers
    pub fn handle_keypress(&mut self, key: &str) -> KeypressResult {
        match self.keymap.lookup(key) {
            Some(Action::GoUp) => {
                if self.nav.move_up() {
                    self.preview.mark_pending();
                }
                KeypressResult::Continue
            }
            Some(Action::GoDown) => {
                if self.nav.move_down() {
                    self.preview.mark_pending();
                }
                KeypressResult::Continue
            }
            Some(Action::GoParent) => self.handle_go_parent(),
            Some(Action::GoIntoDir) => self.handle_go_into_dir(),
            Some(Action::Open) => self.handle_open_file(),
            Some(Action::Quit) => KeypressResult::Quit,
            _ => KeypressResult::Continue,
        }
    }

    fn handle_open_file(&mut self) -> KeypressResult {
        if let Some(entry) = self.nav.selected_entry()
            && !entry.is_dir()
        {
            let path = self.nav.current_dir().join(entry.name());
            if let Err(e) = open_in_editor(self.config.editor(), &path) {
                eprintln!("Error: {}", e);
            }
            return KeypressResult::OpenedEditor;
        }
        KeypressResult::Continue
    }

    fn handle_go_into_dir(&mut self) -> KeypressResult {
        if let Some(entry) = self.nav.selected_entry()
            && entry.is_dir()
        {
            let new_path = self.nav.current_dir().join(entry.name());
            self.nav.save_position();
            self.nav.set_path(new_path);

            self.request_dir_load(None);
            self.request_parent_content();
        }
        KeypressResult::Continue
    }

    fn handle_go_parent(&mut self) -> KeypressResult {
        if let Some(parent) = self.nav.current_dir().parent() {
            let exited_name = self.nav.current_dir().file_name().map(|n| n.to_os_string());
            let parent_path = parent.to_path_buf();

            self.nav.save_position();
            self.nav.set_path(parent_path);

            self.request_dir_load(exited_name);
            self.request_parent_content();
        }
        KeypressResult::Continue
    }

    // Worker request
    pub fn request_dir_load(&mut self, focus: Option<std::ffi::OsString>) {
        self.is_loading = true;
        let request_id = self.nav.prepare_new_request();
        let _ = self.worker_tx.send(WorkerTask::LoadDirectory {
            path: self.nav.current_dir().to_path_buf(),
            focus,
            dirs_first: self.config.dirs_first(),
            show_hidden: self.config.show_hidden(),
            show_system: self.config.show_system(),
            case_insensitive: self.config.case_insensitive(),
            always_show: Arc::clone(self.config.always_show()),
            pane_width: self.metrics.main_width,
            request_id,
        });
    }

    pub fn request_preview(&mut self) {
        if let Some(entry) = self.nav.selected_entry() {
            let path = self.nav.current_dir().join(entry.name());
            let req_id = self.preview.prepare_new_request(path.clone());

            if entry.is_dir() {
                let _ = self.worker_tx.send(WorkerTask::LoadDirectory {
                    path,
                    focus: None,
                    dirs_first: self.config.dirs_first(),
                    show_hidden: self.config.show_hidden(),
                    show_system: self.config.show_system(),
                    case_insensitive: self.config.case_insensitive(),
                    always_show: Arc::clone(self.config.always_show()),
                    pane_width: self.metrics.preview_width,
                    request_id: req_id,
                });
            } else {
                let _ = self.worker_tx.send(WorkerTask::LoadPreview {
                    path,
                    max_lines: self.metrics.preview_height,
                    pane_width: self.metrics.preview_width,
                    request_id: req_id,
                });
            }
        }
    }

    pub fn request_parent_content(&mut self) {
        if let Some(parent_path) = self.nav.current_dir().parent() {
            let parent_path_buf = parent_path.to_path_buf();

            if self.parent.should_request(&parent_path_buf) {
                let req_id = self.parent.prepare_new_request(parent_path_buf.clone());

                let _ = self.worker_tx.send(WorkerTask::LoadDirectory {
                    path: parent_path_buf,
                    focus: None,
                    dirs_first: self.config.dirs_first(),
                    show_hidden: self.config.show_hidden(),
                    show_system: self.config.show_system(),
                    case_insensitive: self.config.case_insensitive(),
                    always_show: Arc::clone(self.config.always_show()),
                    pane_width: self.metrics.parent_width,
                    request_id: req_id,
                });
            }
        } else {
            // at root.
            self.parent.clear();
        }
    }
}
