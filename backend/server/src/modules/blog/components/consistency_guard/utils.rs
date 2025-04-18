use crate::modules::blog::components::consistency_guard::error::Error;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn rec_find_dir_entries(entry: &Path) -> Result<Vec<PathBuf>, Error> {
    if !entry.exists() {
        return Err(Error::InvalidPath(entry.to_path_buf()));
    }

    let mut result = vec![];

    if !entry.is_dir() {
        return Ok(vec![entry.to_path_buf()]);
    }

    for entry in fs::read_dir(entry)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            result.extend(rec_find_dir_entries(&path)?);
        } else {
            result.push(path);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_valid_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        fs::create_dir(dir_path.join("subdir")).unwrap();
        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("subdir/file2.txt")).unwrap();

        let entries = rec_find_dir_entries(dir_path).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_non_existent_path() {
        let non_existent = PathBuf::from("/non/existent/path");
        assert!(matches!(
            rec_find_dir_entries(&non_existent),
            Err(Error::InvalidPath(_))
        ));
    }

    #[test]
    fn test_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let entries = rec_find_dir_entries(&file_path).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], file_path);
    }

    #[test]
    fn test_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let entries = rec_find_dir_entries(temp_dir.path()).unwrap();
        assert!(entries.is_empty());
    }
}
