use std::fs;
use std::io;
use std::path::Path;

/// Ensure a directory exists, creating it with parents if necessary.
pub fn ensure_dir_exists(path: &Path) -> Result<(), io::Error> {
    if !path.is_dir() {
        if path.exists() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Not a directory."));
        }
        fs::create_dir_all(&path)?;
    }
    Ok(())
}
