pub fn bucle_cuando() {
    let numeros = vec![1, 2, 3, 4, 5];
    for numero in &numeros {
        // Intentamos modificar el valor que estamos iterando
        *numero += 1; // Esto generará un error de tipo
        println!("El número es: {}", numero);
    }
}
