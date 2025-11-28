use std::fs;
use std::path::PathBuf;

pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

pub fn read_dir(path: &str) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    for entry_result in fs::read_dir(path)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
            is_dir: file_type.is_dir(),
        });
    }
    Ok(entries)
}
