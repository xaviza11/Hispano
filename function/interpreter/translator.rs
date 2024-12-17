pub const DICTIONARY: [(&str, &str); 117] = [
    ("como", "as"),
    ("romper", "break"),
    ("const", "const"),
    ("continuar", "continue"),
    ("caja", "crate"),
    ("sino", "else"),
    ("enumerar", "enum"),
    ("externo", "extern"),
    ("falso", "false"),
    ("funcion", "fn"),
    ("cuando", "for"),
    ("si", "if"),
    ("implementar", "impl"),
    ("en", "in"),
    ("var", "let"),
    ("siempre", "loop"),
    ("coincide", "match"),
    ("modulo", "mod"),
    ("mover", "move"),
    ("mut", "mut"),
    ("pub", "pub"),
    ("ref", "ref"),
    ("retornar", "return"),
    ("Elmismo", "Self"),
    ("elmismo", "self"),
    ("estatico", "static"),
    ("estructura", "struct"),
    ("superior", "super"),
    ("rasgo", "trait"),
    ("verdadero", "true"),
    ("tipo", "type"),
    ("inseguro", "unsafe"),
    ("usar", "use"),
    ("donde", "where"),
    ("mientras", "while"),
    ("abstracto", "abstract"),
    ("asincrono", "async"),
    ("esperar", "await"),
    ("convertirse", "become"),
    ("caja", "box"),
    ("hacer", "do"),
    ("final", "final"),
    ("macro", "macro"),
    ("sobrescribir", "override"),
    ("privado", "priv"),
    ("intentar", "try"),
    ("tipode", "typeof"),
    ("tamañodinamico", "unsized"),
    ("virtual", "virtual"),
    ("ceder", "yield"),
    ("imprimir", "println!"),
    ("automatico", "auto"),
    ("capturar", "catch"),
    ("defecto", "default"),
    ("dinamico", "dyn"),
    ("union", "union"),
    ("reglas_macro", "macro_rules"),
    ("jefe", "main"),
    ("e32", "i32"),
    ("e64", "i64"),
    ("e16", "i16"),
    ("e8", "i8"),
    ("in32", "u32"),
    ("in64", "u64"),
    ("in16", "u16"),
    ("in8", "u8"),
    ("caracter", "char"),
    ("Cadena", "String"),
    ("cadena", "str"),
    ("booleano", "bool"),
    ("nueva", "new"),
    ("filtrar", "filter"),
    ("recortar", "trim"),
    ("mapear", "map"),
    ("reducir", "reduce"),
    ("clonar", "clone"),
    ("concatenar", "concat"),
    ("vacio", "is_empty"),
    ("longitud", "len"),
    ("contenido", "contains"),
    ("buscar", "find"),
    ("ordenar", "sort"),
    ("leer", "read"),
    ("escribir", "write"),
    ("cambiar", "change"),
    ("pasar", "to"),
    ("obtener", "get"),
    ("establecer", "set"),
    ("primero", "first"),
    ("ultimo", "last"),
    ("enviar", "send"),
    ("recibir", "recv"),
    ("completar", "complete"),
    ("cerrar", "close"),
    ("iterar", "iter"),
    ("dividir", "split"),
    ("unir", "join"),
    ("reemplazar", "replace"),
    ("escribir_arc", "write_file"),
    ("leer_fichero", "read_file"),
    ("agregar", "push"),
    ("quitar", "pop"),
    ("insertar", "insert"),
    ("tarea", "task"),
    ("guardar", "store"),
    ("leer_entrada", "read_input"),
    ("comprobar", "check"),
    ("establecer_tmp", "set_timeout"),
    ("retrasar", "delay"),
    ("almacenar", "store"),
    ("suspender", "suspend"),
    ("cualquiera", "any"),
    ("panico", "panic"),
    ("afirmo", "assert"),
    ("afirmo_no", "assert_ne"),
    ("afirmo_igual", "assert_eq"),
    ("otro", "another")
];

pub fn translator(tokens: &Vec<String>) -> String {
    let mut translated_tokens: Vec<String> = Vec::new();
    let mut inside_comment = false;

    let mut prev_token = ""; // Variable para almacenar el token anterior y detectar "/*"

    for token in tokens.iter() {
        if prev_token == "/" && token == "*" {
            inside_comment = true;
        }
        if prev_token == "*" && token == "/" {
            inside_comment = false;
        }

        if inside_comment {
            translated_tokens.push(token.to_string());
        } else {
            let translated_token = DICTIONARY.iter()
                .find(|&&(key, _)| key == *token)
                .map(|&(_, value)| value.to_string())
                .unwrap_or_else(|| token.to_string());

            translated_tokens.push(translated_token);
        }

        prev_token = token;
    }

    translated_tokens.join(" ")
}
