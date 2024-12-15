/// This function formats a vector of tokens into a single string.
pub fn formatter(tokens: Vec<&str>) -> String {
    // Join all tokens without adding extra spaces
    let result = tokens.join("");

    // Replace "MARK_SPACE" with an actual space
    result.replace("MARK_SPACE", " ")
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_basic_tokens() {
        let tokens = vec!["let", "x", "=", "10", "+", "20", ";"];
        let expected = "letx=10+20;";
        let result = formatter(tokens);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_formatter_with_spaces() {
        let tokens = vec!["let", "MARK_SPACE", "x", "MARK_SPACE", "=", "MARK_SPACE", "10", "MARK_SPACE", "+", "MARK_SPACE", "20", ";"];
        let expected = "let x = 10 + 20;";
        let result = formatter(tokens);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_formatter_with_no_tokens() {
        let tokens: Vec<&str> = Vec::new();
        let expected = "";
        let result = formatter(tokens);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_formatter_with_only_spaces() {
        let tokens = vec!["MARK_SPACE", "MARK_SPACE", "MARK_SPACE"];
        let expected = "   ";
        let result = formatter(tokens);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_formatter_mixed_tokens_and_spaces() {
        let tokens = vec!["if", "MARK_SPACE", "x", "MARK_SPACE", ">", "MARK_SPACE", "5", "MARK_SPACE", "{", "MARK_SPACE", "println!"];
        let expected = "if x > 5 { println!";
        let result = formatter(tokens);
        assert_eq!(result, expected);
    }
}*/
