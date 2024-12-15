use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Creates a new file with the specified content, ensuring necessary directories exist.
///
/// If the file name ends with `.his.rs`, it replaces the extension with `.rs`. The file
/// is saved inside the "output" folder.
///
/// # Arguments
/// * `path` - A string slice that represents the file path.
/// * `content` - A string slice that contains the content to write into the file.
///
/// # Returns
/// Returns `io::Result` indicating success or failure.
pub fn creator(path: &str, content: &str) -> io::Result<()> {
    // Prepare the file name
    let path = if path.ends_with(".his.rs") {
        path.replace(".his.rs", ".rs") // Replace .his.rs with .rs
    } else {
        path.to_string() // Leave the path unchanged
    };

    // Prepare the output path inside the "output" folder
    let output_path = format!("./output/{}", path);
    let path = Path::new(&output_path);

    // If the file belongs to a folder, create all necessary parent directories
    if let Some(parent_folder) = path.parent() {
        fs::create_dir_all(parent_folder)?;
    }

    // Create and open the file for writing
    if path.extension() == Some(std::ffi::OsStr::new("rs")) {
        let mut output_file = File::create(&output_path)?;
        write!(output_file, "{}", content)?;
    }

    println!("Archivo creado: {}", output_path);
    Ok(())
}
