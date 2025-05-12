use std::path::{Path, PathBuf};
use std::fs;
use std::io;

pub fn init_log<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    let file_path = file_path.as_ref();
    let file_stem = file_path.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file name"))?
        .to_string_lossy();

    let parent_dir = file_path.parent().unwrap_or_else(|| Path::new("."));

    let log_folder = parent_dir.join(format!(".{}_log", file_stem));
    fs::create_dir_all(&log_folder)?;

    let file_name = file_path.file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file name"))?;
    let destination = log_folder.join(file_name);
    fs::copy(file_path, destination)?;

    Ok(())
}