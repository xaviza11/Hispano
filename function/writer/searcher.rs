use std::fs;
use std::path::Path;
use std::io;

/// Recursively searches for files in a given directory and its subdirectories.
///
/// # Arguments
/// * `directory_path` - The path of the directory to search.
/// * `files` - A mutable reference to a vector where file paths will be stored.
///
/// # Returns
/// Returns an `io::Result` indicating success or failure.
pub fn searcher(directory_path: &str, files: &mut Vec<String>) -> io::Result<()> {
    let entries = fs::read_dir(directory_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively search the subdirectory
            searcher(&path.to_string_lossy(), files)?;
        } else if path.is_file() {
            println!("Archivo: {}", path.display());
            files.push(path.to_string_lossy().to_string());
        }
    }

    Ok(())
}

/// Initiates the search for files starting from the "app" directory.
///
/// # Returns
/// Returns a vector of file paths or an `io::Result` if an error occurs.
pub fn search() -> io::Result<Vec<String>> {
    let dir = "app";
    let mut files = Vec::new();

    if Path::new(dir).exists() {
        searcher(dir, &mut files)?;
        println!("Listado completado.");
    } else {
        eprintln!("El directorio '{}' no existe!", dir);
    }

    Ok(files)
}