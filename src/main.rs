mod file_manager;

fn main() -> std::io::Result<()> {
    let entries = file_manager::read_dir(".")?;
    for entry in entries {
        if entry.is_dir {
            println!("DIR:      {}", entry.name);
        } else {
            println!("          {}", entry.name);
        }
    }
    Ok(())
}
