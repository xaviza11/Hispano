mod ejemplos;fn main() {    ejemplos::condicional_coincide::condicional_coincide();    ejemplos::condicional_si::condicional_si();        ejemplos::bucle_cuando::bucle_cuando();    ejemplos::bucle_mientras::bucle_mientras();    ejemplos::bucle_siempre::bucle_siempre();}#[cfg(test)]mod tests {    #[test]    fn test_inicial() {        assert!(true);    }}