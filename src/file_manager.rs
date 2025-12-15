use std::ffi::OsString;
use std::fs;

pub struct FileEntry {
    pub name: OsString,
    pub is_dir: bool,
    pub is_hidden: bool,
}

pub fn browse_dir(path: &std::path::Path) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::with_capacity(256);
    for entry in fs::read_dir(path)? {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let name = entry.file_name();
        #[cfg(unix)]
        let (is_dir, is_hidden) = {
            use std::os::unix::ffi::OsStrExt;
            let is_hidden = name.as_bytes().get(0) == Some(&b'.');
            let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            (is_dir, is_hidden)
        };

        #[cfg(windows)]
        let (is_dir, is_hidden) = {
            use std::os::windows::fs::MetadataExt;

            if let Ok(md) = entry.metadata() {
                let is_hidden = md.file_attributes() & 0x2 != 0;
                let is_dir = md.is_dir();
                (is_dir, is_hidden)
            } else {
                (false, false)
            }
        };
        entries.push(FileEntry {
            name,
            is_dir,
            is_hidden,
        });
    }
    Ok(entries)
}
