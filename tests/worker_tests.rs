use crossbeam_channel::unbounded;
use runa_tui::file_manager;
use runa_tui::formatter::Formatter;
use runa_tui::worker::{WorkerResponse, WorkerTask, start_worker};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

#[test]
fn test_worker_load_current_dir() {
    let (task_tx, task_rx) = unbounded();
    let (res_tx, res_rx) = unbounded();

    start_worker(task_rx, res_tx);

    task_tx
        .send(WorkerTask::LoadDirectory {
            path: std::env::current_dir().unwrap(),
            focus: None,
            dirs_first: true,
            show_hidden: false,
            show_system: false,
            case_insensitive: true,
            always_show: Arc::new(HashSet::new()),
            pane_width: 20,
            request_id: 1,
        })
        .unwrap();

    match res_rx.recv() {
        Ok(WorkerResponse::DirectoryLoaded { entries, .. }) => {
            assert!(!entries.is_empty(), "Current dir should not be empty");

            // Check display name width
            for entry in entries {
                let disp = entry.display_name();
                assert!(
                    disp.chars().count() <= 20,
                    "Entry '{}' too wide",
                    entry.name_str()
                );
            }
        }
        Ok(WorkerResponse::Error(e)) => panic!("Worker error: {}", e),
        _ => panic!("Unexpected worker response"),
    }
}

#[test]
fn test_formatter_truncation() {
    let width = 5;
    let formatter = Formatter::new(true, true, true, false, Arc::new(HashSet::new()), width);

    let path = Path::new(".");
    let mut entries = file_manager::browse_dir(path).expect("Failed to read dir");

    formatter.format(&mut entries);

    for entry in entries {
        let disp = entry.display_name();
        assert!(
            disp.chars().count() <= width,
            "Entry '{}' not truncated properly",
            entry.name_str()
        );
    }
}
