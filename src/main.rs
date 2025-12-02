mod file_manager;
mod formatter;

// use crate::file_manager::FileEntry;
// use crate::formatter::Formatter;

fn main() -> std::io::Result<()> {
    let mut entries = file_manager::read_dir(".")?;
    let formatter = formatter::Formatter::new(true);
    formatter.format(&mut entries);
    for entry in entries {
        println!("{}", entry.name);
    }
    Ok(())
}
