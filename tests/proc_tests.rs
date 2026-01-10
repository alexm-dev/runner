//! Tests for the sub process functionality in runa_tui.
//! These tests require the `fd` command-line tool to be installed.
//! If `fd` is not available, the tests will be skipped.
//! If `bat` is not available, the tests will be skipped

use runa_tui::core::{find, preview_bat};
use std::fs;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tempfile::tempdir;

/// Checks if the `fd` command-line tool is available in the system.
/// Returns true if `fd` is found, otherwise false.
/// Uses which crate to check for the presence of `fd`.
fn fd_available() -> bool {
    which::which("fd").is_ok()
}

fn bat_available() -> bool {
    which::which("bat").is_ok()
}

/// Macro to skip tests if `fd` is not available.
macro_rules! skip_if_no_fd {
    () => {
        if !fd_available() {
            return Ok(());
        }
    };
}

/// Macro to skip tests if `bat` is not available.
macro_rules! skip_if_no_bat {
    () => {
        if !bat_available() {
            return Ok(());
        }
    };
}

#[test]
fn test_find_recursive_unit() -> Result<(), Box<dyn std::error::Error>> {
    skip_if_no_fd!();

    let dir = tempdir()?;
    std::fs::File::create(dir.path().join("crab.txt"))?;
    std::fs::File::create(dir.path().join("other.txt"))?;
    let cancel = Arc::new(AtomicBool::new(false));
    let mut out = Vec::new();
    find(dir.path(), "crab", &mut out, cancel, 11)?;
    let candidate = out
        .iter()
        .find(|r| r.path().file_name().unwrap() == "crab.txt");
    assert!(
        candidate.is_some(),
        "Expected 'crab.txt' in find results. Got: {:?}",
        out.iter()
            .map(|r| r.path().display().to_string())
            .collect::<Vec<_>>()
    );

    let filename = out[0]
        .path()
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("Could not extract valid UTF-8 file name")?;
    assert!(
        filename.contains("crab"),
        "Filename does not contain 'crab': got '{}'",
        filename
    );
    Ok(())
}

#[test]
fn test_find_recursive_empty_query() -> Result<(), Box<dyn std::error::Error>> {
    skip_if_no_fd!();
    let dir = tempdir()?;
    fs::File::create(dir.path().join("something.txt"))?;
    let cancel = Arc::new(AtomicBool::new(false));
    let mut out = Vec::new();
    find(dir.path(), "", &mut out, cancel, 10)?;
    assert!(out.is_empty());
    Ok(())
}

#[test]
fn test_find_recursive_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
    skip_if_no_fd!();
    let dir = tempdir()?;
    let subdir = dir.path().join("nested");
    std::fs::create_dir(&subdir)?;
    std::fs::File::create(subdir.join("crabby.rs"))?;
    let cancel = Arc::new(AtomicBool::new(false));
    let mut out = Vec::new();
    find(dir.path(), "crabby", &mut out, cancel, 10)?;
    let candidate = out
        .iter()
        .find(|r| r.path().file_name().unwrap() == "crabby.rs");
    assert!(
        candidate.is_some(),
        "Expected 'crabby.rs' in find results. Got: {:?}",
        out.iter()
            .map(|r| r.path().display().to_string())
            .collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn test_preview_bat_basic() -> Result<(), Box<dyn std::error::Error>> {
    skip_if_no_bat!();

    let dir = tempdir()?;
    let file_path = dir.path().join("hello.txt");
    let mut file = fs::File::create(&file_path)?;
    writeln!(file, "line one")?;
    writeln!(file, "line two")?;
    writeln!(file, "line three")?;

    let preview = preview_bat(&file_path, 2, &[])?;
    assert_eq!(preview.len(), 2);
    assert!(
        preview.iter().any(|line| line.contains("line one")),
        "Preview missing expected content"
    );
    assert!(
        preview.iter().any(|line| line.contains("line two")),
        "Preview missing expected content"
    );

    Ok(())
}

#[test]
fn test_preview_bat_with_args() -> Result<(), Box<dyn std::error::Error>> {
    skip_if_no_bat!();

    let dir = tempdir()?;
    let file_path = dir.path().join("colors.rs");
    let mut file = fs::File::create(&file_path)?;
    writeln!(file, "fn main() {{}}")?;

    let preview = preview_bat(&file_path, 1, &[std::ffi::OsString::from("--plain")])?;
    assert_eq!(preview.len(), 1);
    assert!(preview[0].contains("fn main"));

    Ok(())
}

#[test]
fn test_preview_bat_nonexistent_file() -> Result<(), Box<dyn std::error::Error>> {
    skip_if_no_bat!();

    let dir = tempfile::tempdir()?;
    let bad_path = dir.path().join("does_not_exist.txt");
    let result = preview_bat(&bad_path, 2, &[]);

    assert!(result.is_err(), "Expected error for missing file");
    Ok(())
}
