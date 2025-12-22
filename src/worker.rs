use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{Receiver, Sender};
use unicode_width::UnicodeWidthChar;

use crate::file_manager::{FileEntry, browse_dir};
use crate::formatter::Formatter;

pub enum WorkerTask {
    LoadDirectory {
        path: PathBuf,
        focus: Option<OsString>,
        dirs_first: bool,
        show_hidden: bool,
        show_system: bool,
        case_insensitive: bool,
        always_show: Arc<HashSet<OsString>>,
        pane_width: usize,
        request_id: u64,
    },
    LoadPreview {
        path: PathBuf,
        max_lines: usize,
        pane_width: usize,
        request_id: u64,
    },
}

pub enum WorkerResponse {
    DirectoryLoaded {
        path: PathBuf,
        entries: Vec<FileEntry>,
        focus: Option<OsString>,
        request_id: u64,
    },
    PreviewLoaded {
        lines: Vec<String>,
        request_id: u64,
    },
    Error(String),
}

pub fn start_worker(task_rx: Receiver<WorkerTask>, res_tx: Sender<WorkerResponse>) {
    thread::spawn(move || {
        while let Ok(task) = task_rx.recv() {
            match task {
                WorkerTask::LoadDirectory {
                    path,
                    focus,
                    dirs_first,
                    show_hidden,
                    show_system,
                    case_insensitive,
                    always_show,
                    pane_width,
                    request_id,
                } => match browse_dir(&path) {
                    Ok(mut entries) => {
                        let formatter = Formatter::new(
                            dirs_first,
                            show_hidden,
                            show_system,
                            case_insensitive,
                            always_show,
                            pane_width,
                        );
                        formatter.filter_entries(&mut entries);
                        let _ = res_tx.send(WorkerResponse::DirectoryLoaded {
                            path,
                            entries,
                            focus,
                            request_id,
                        });
                    }
                    Err(e) => {
                        let _ = res_tx.send(WorkerResponse::Error(format!("I/O Error: {}", e)));
                    }
                },
                WorkerTask::LoadPreview {
                    path,
                    max_lines,
                    pane_width,
                    request_id,
                } => {
                    let lines = safe_read_preview(&path, max_lines, pane_width);
                    let _ = res_tx.send(WorkerResponse::PreviewLoaded { lines, request_id });
                }
            }
        }
    });
}

// Calculating the pane widht and clean the output to the widht of the pane
fn sanitize_to_exact_width(line: &str, pane_width: usize) -> String {
    let mut out = String::with_capacity(pane_width);
    let mut current_w = 0;

    for char in line.chars() {
        if char == '\t' {
            let space_count = 4 - (current_w % 4);
            if current_w + space_count > pane_width {
                break;
            }
            out.push_str(&" ".repeat(space_count));
            current_w += space_count;
            continue;
        }

        if char.is_control() {
            continue;
        }

        let w = char.width().unwrap_or(0);
        if current_w + w > pane_width {
            break;
        }

        out.push(char);
        current_w += w;
    }

    // If the string is shorter than the pane, fill it with spaces.
    if current_w < pane_width {
        out.push_str(&" ".repeat(pane_width - current_w));
    }

    out
}

fn preview_directory(path: &Path, max_lines: usize, pane_width: usize) -> Vec<String> {
    match browse_dir(path) {
        Ok(entries) => {
            let mut lines = Vec::with_capacity(max_lines + 1);

            // Process existing entries
            for e in entries.iter().take(max_lines) {
                let suffix = if e.is_dir() { "/" } else { "" };
                let display_name = format!("{}{}", e.name().to_string_lossy(), suffix);

                // Sanitize and pad to exact width
                lines.push(sanitize_to_exact_width(&display_name, pane_width));
            }

            // Handle Empty State
            if lines.is_empty() {
                lines.push(sanitize_to_exact_width("[empty directory]", pane_width));
            }
            // Handle Overflow Indicator
            else if entries.len() > max_lines {
                lines.pop();
                lines.push(sanitize_to_exact_width("...", pane_width));
            }

            // If the folder has fewer items than the height of the pane,
            // it fills the remaining lines with empty padded strings.
            // This physically erases old content from the bottom of the pane.
            while lines.len() < max_lines {
                lines.push(" ".repeat(pane_width));
            }

            lines
        }
        Err(e) => {
            let mut err_lines = vec![sanitize_to_exact_width(
                &format!("[Error: {}]", e),
                pane_width,
            )];
            // Fill error screen with blanks too
            while err_lines.len() < max_lines {
                err_lines.push(" ".repeat(pane_width));
            }
            err_lines
        }
    }
}

fn safe_read_preview(path: &Path, max_lines: usize, pane_width: usize) -> Vec<String> {
    let max_lines = std::cmp::max(max_lines, 3);

    // Metadata check
    let Ok(meta) = std::fs::metadata(path) else {
        return vec![sanitize_to_exact_width(
            "[Error: Access Denied]",
            pane_width,
        )];
    };

    if path.is_dir() {
        return preview_directory(path, max_lines, pane_width);
    }

    // Size Check
    const MAX_PREVIEW_SIZE: u64 = 10 * 1024 * 1024;
    if meta.len() > MAX_PREVIEW_SIZE {
        return vec![sanitize_to_exact_width(
            "[File too large for preview]",
            pane_width,
        )];
    }

    if !meta.is_file() {
        return vec![sanitize_to_exact_width("[Not a regular file]", pane_width)];
    }

    // File Read and binary Check
    match File::open(path) {
        Ok(mut file) => {
            // First, peek for null bytes to detect binary files
            let mut buffer = [0u8; 1024];
            let n = file.read(&mut buffer).unwrap_or(0);
            if buffer[..n].contains(&0) {
                return vec![sanitize_to_exact_width(
                    "[Binary file - preview hidden]",
                    pane_width,
                )];
            }

            let _ = file.rewind();

            let reader = BufReader::new(file);
            let mut preview_lines = Vec::with_capacity(max_lines);

            for line_result in reader.lines().take(max_lines) {
                match line_result {
                    Ok(line) => {
                        preview_lines.push(sanitize_to_exact_width(&line, pane_width));
                    }
                    Err(_) => break,
                }
            }

            if preview_lines.is_empty() {
                preview_lines.push(sanitize_to_exact_width("[Empty file]", pane_width));
            }

            preview_lines
        }
        Err(e) => vec![sanitize_to_exact_width(
            &format!("[Error reading file: {}]", e),
            pane_width,
        )],
    }
}
