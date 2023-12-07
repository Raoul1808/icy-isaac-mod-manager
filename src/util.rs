use std::{fs::File, io, path::PathBuf};

pub fn create_empty_file(path: PathBuf) -> io::Result<()> {
    let _ = File::create(path)?;
    Ok(())
}

pub fn get_config_dir() -> Option<PathBuf> {
    let dir = directories::BaseDirs::new()?
        .config_dir()
        .join("IcyIsaacModManager");
    Some(dir)
}
