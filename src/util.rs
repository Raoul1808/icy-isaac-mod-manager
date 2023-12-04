use std::{fs::File, io, path::PathBuf};

pub fn create_empty_file(path: PathBuf) -> io::Result<()> {
    let _ = File::create(path)?;
    Ok(())
}
