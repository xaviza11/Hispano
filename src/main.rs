use clap::{Command};
use std::process::{Command as ProcessCommand, ExitStatus};

fn main() {
    let matches = Command::new("hispano")
        .version("1.0")
        .author("Tu Nombre <xaviza11@gmail.com>")
        .about("Herramienta de comandos personalizados para proyectos Rust")
        .subcommand(Command::new("construir").about("Ejecuta el binario function_bin"))
        .subcommand(Command::new("correr").about("Ejecuta el binario output_bin"))
        .subcommand(Command::new("alzar").about("Construye y corre el binario output_bin"))
        .subcommand(Command::new("comandos").about("Muestra los comandos disponibles"))
        .subcommand(Command::new("test").about("Ejecuta los tests del proyecto usando cargo test"))
        .get_matches();

    match matches.subcommand() {
        Some(("construir", _)) => {
            println!("Transpilando el código...");
            if let Err(e) = run_command("cargo", &["run", "--bin", "function_bin"]) {
                eprintln!("{}", e);
            }
        }
        Some(("correr", _)) => {
            println!("Corriendo el programa...");
            if let Err(e) = run_command("cargo", &["run", "--bin", "output_bin"]) {
                eprintln!("{}", e);
            }
        }
        Some(("alzar", _)) => {
            println!("Construyendo y corriendo la aplicación...");
            // Primero construimos el proyecto
            if let Err(e) = run_command("cargo", &["run", "--bin", "function_bin"]) {
                eprintln!("{}", e);
            } else {
                // Si la construcción fue exitosa, procedemos a correr el binario
                if let Err(e) = run_command("cargo", &["run", "--bin", "output_bin"]) {
                    eprintln!("{}", e);
                }
            }
        }
        Some(("comandos", _)) => {
            println!("Comandos disponibles:");
            println!("  hispano construir  - Construye la aplicación");
            println!("  hispano correr     - Corre la aplicación");
            println!("  hispano alzar      - Construye y corre la aplicación");
            println!("  hispano test       - Ejecuta los tests del proyecto");
            println!("  hispano comandos   - Muestra esta lista de comandos");
        }
        Some(("test", _)) => {
            println!("Ejecutando pruebas del proyecto...");
            if let Err(e) = run_command("cargo", &["test"]) {
                eprintln!("{}", e);
            }
        }
        _ => {
            eprintln!("Comando no reconocido. Usa 'hispano construir', 'hispano correr', 'hispano test', 'hispano alzar' o 'hispano comandos'.");
        }
    }
}

// Ahora, `run_command` devuelve un Result con un mensaje de error si el comando falla.
fn run_command(command: &str, args: &[&str]) -> Result<ExitStatus, String> {
    let status = ProcessCommand::new(command)
        .args(args)
        .status()
        .map_err(|e| format!("Error al ejecutar el comando '{}': {}", command, e))?;
    
    if !status.success() {
        return Err(format!("El comando '{}' falló con código de salida: {}", command, status));
    }
    Ok(status)
}
