pub fn main () {    const ENTERO: i32 = -42;    const SIN_SIGNO: u64 = 100000;    const FLOTANTE: f64 = 3.14159;     const CARACTER: char = "A";     const BOLEANO: bool = true;     const TUPLA: (i32, f64, char) = (10, 3.14, "R");     const ARREGLO: [i32; 3] = [1, 2, 3];        const (x, y, z) = TUPLA;     println!("Tupla: x = {}, y = {}, z = {}", x, y, z);    println!("ARREGLO: primer elemento = {}", ARREGLO[0]);    println!("Tipo Escalar - Entero: {}", ENTERO);    println!("Tipo Escalar - Sin signo: {}", SIN_SIGNO);    println!("Tipo Escalar - Flotante: {}", FLOTANTE);    println!("Tipo Escalar - Caracter: {}", CARACTER);    println!("Tipo Escalar - Booleano: {}", BOLEANO);}