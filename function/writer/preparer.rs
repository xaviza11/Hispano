use std::path::{Path, PathBuf};

/// Prepares a new path by removing the "app" prefix and changing the file extension to ".rs".
///
/// # Arguments
/// * `file_path` - A string slice that holds the file path to process.
///
/// # Returns
/// Returns a `PathBuf` with the modified path.
pub fn preparer(file_path: &str) -> PathBuf {
    let relative_path = Path::new(file_path)
        .strip_prefix("app")
        .unwrap_or_else(|_| Path::new(file_path));

    relative_path.with_extension("rs")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_preparer_removes_app_prefix_and_changes_extension() {
        let input_path = "app/subdir/file.his";
        let expected_output = PathBuf::from("subdir/file.rs");
        let result = preparer(input_path);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_preparer_no_app_prefix_changes_extension() {
        let input_path = "otherdir/file.his";
        let expected_output = PathBuf::from("otherdir/file.rs");
        let result = preparer(input_path);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_preparer_root_file_changes_extension() {
        let input_path = "file.his";
        let expected_output = PathBuf::from("file.rs");
        let result = preparer(input_path);
        assert_eq!(result, expected_output);
    }
}
