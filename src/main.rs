mod file_manager;
mod formatter;

// use crate::file_manager::FileEntry;
// use crate::formatter::Formatter;

fn main() -> std::io::Result<()> {
    let entries = file_manager::read_dir(".")?;
    let formatter = formatter::Formatter::new(true);
    let sorted = formatter.format(entries);
    for entry in sorted {
        println!("{}", entry.name);
    }
    Ok(())
}
