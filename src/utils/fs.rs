use std::fs;
use std::io;
use std::path;

/// Ensure a directory exists, creating it with parents if necessary.
pub fn ensure_dir_exists(path: &path::Path) -> Result<(), io::Error> {
    if !path.is_dir() {
        if path.exists() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Not a directory."));
        }
        fs::create_dir_all(&path)?;
    }
    Ok(())
}

/// Strip the extension from a file path.
pub fn strip_extension(path: &path::PathBuf) -> Option<path::PathBuf> {
    let mut pb = path::PathBuf::from(&path);
    if pb.extension().is_none() {
        eprintln!("Path has no extension: {:?}", &path);
        return None
    }
    pb.set_extension("");
    Some(pb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_extension() {
        let pb = path::PathBuf::from("file.ext");
        assert_eq!(strip_extension(&pb).unwrap(), path::PathBuf::from("file"));
        let pb = path::PathBuf::from("file");
        assert!(strip_extension(&pb).is_none());
    }
}
