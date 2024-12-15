use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::Path;

mod interpreter;
mod writer;

fn main() -> io::Result<()> {
    // Obtain the list of files using the searcher
    let files = writer::searcher::search()?;

    println!("Archivos encontrados: ");
    for file_path in &files {
        // Process files that end with ".his"
        if file_path.ends_with(".his") {
            println!("{}", file_path);

            let file = File::open(file_path)?;
            let mut parser = BufReader::new(file);

            let mut content = String::new();
            parser.read_to_string(&mut content)?;

            // Process the content using the interpreter functions
            let result = interpreter::parser::parser(&content);
            let translation = interpreter::translator::translator(&result);
            let tokens: Vec<&str> = translation.split_whitespace().collect();
            let formatted = interpreter::formatter::formatter(tokens);

            // Prepare the path inside "output" using the new function
            let mut new_file_path = writer::preparer::preparer(file_path);

            // If the file is named "jefe.his", rename the output to "main.rs"
            if Path::new(file_path).file_name() == Some(std::ffi::OsStr::new("jefe.his")) {
                new_file_path.set_file_name("main.rs");
            }

            // Call the function to create folders and write the formatted file
            writer::creator::creator(new_file_path.to_str().unwrap(), &formatted)?;
        } else {
            // If the file does not end with ".his", copy it as is to the output folder
            println!("Copiando archivo: {}", file_path);

            let input_path = Path::new(file_path);

            // Remove the "/app" prefix from the path
            let output_path = input_path.strip_prefix("app")
                .map(|p| format!("./output/{}", p.display()))
                .unwrap_or_else(|_| format!("./output/{}", file_path));

            let output_path = Path::new(&output_path);

            // If the file belongs to a folder, create all necessary directories
            if let Some(parent_folder) = output_path.parent() {
                fs::create_dir_all(parent_folder)?;
            }

            // Copy the original file to the output folder
            fs::copy(input_path, output_path)?;
        }
    }

    println!("Processo completado archivos generados en  ./output/");
    Ok(())
}
