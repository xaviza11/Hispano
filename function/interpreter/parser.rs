use regex::Regex;

/// This function parses a line of code and splits it into tokens.
pub fn parser(line: &str) -> Vec<String> {
    // Replace spaces with a placeholder "MARK_SPACE"
    let line_with_markers = line.replace(" ", " MARK_SPACE ");

    // Regular expression to match different patterns (strings, numbers, identifiers, operators, etc.)
    let re = Regex::new(
        r#""[^"]*"|\d+|[a-zA-Z_][a-zA-Z0-9_]*|==|#|!|:|!=|>=|á|é|í|ó|ú|ä|ë|ï|ö|ü|Á|É|Í|Ó|Ú|Ä|Ë|Ï|Ö|Ü|ñ|<=|\+=|-=|\*=|/=|%=|\+\+|--|=|[(),;{}\[\].]|>|<|\+|-|\*|/|%|&|\||\^|~|\?|MARK_SPACE"#
    )
    .unwrap();

    // Find all matches of the regular expression in the line and return them as a vector of strings
    let tokens: Vec<String> = re
        .find_iter(&line_with_markers)
        .filter_map(|mat| {
            let token = mat.as_str();
            if token == "MARK_SPACE" {
                Some("MARK_SPACE".to_string()) // Return "MARK_SPACE" for spaces
            } else {
                Some(token.to_string()) // Return the matched token
            }
        })
        .collect();

    tokens
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basic_tokens() {
        let line = "let x = 10 + 20;";
        let expected = vec![
            "let".to_string(), "MARK_SPACE".to_string(), "x".to_string(), "MARK_SPACE".to_string(), "=".to_string(), "MARK_SPACE".to_string(),
            "10".to_string(), "MARK_SPACE".to_string(), "+".to_string(), "MARK_SPACE".to_string(), "20".to_string(), ";".to_string(),
        ];
        let result = parser(line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parser_with_spaces() {
        let line = "if x > 5 { println!(\"x is greater than 5\"); }";
        let expected = vec!["if", "MARK_SPACE", "x", "MARK_SPACE", ">", "MARK_SPACE", "5", "MARK_SPACE", "{", "MARK_SPACE", "println", "!", "(", "\"x MARK_SPACE is MARK_SPACE greater MARK_SPACE than MARK_SPACE 5\"", ")", ";", "MARK_SPACE", "}"];
        let result = parser(line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parser_empty_line() {
        let line = "";
        let expected: Vec<String> = Vec::new();
        let result = parser(line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parser_with_special_characters() {
        let line = "let result = 10 * (5 + 3);";
        let expected = vec![
            "let".to_string(), "MARK_SPACE".to_string(), "result".to_string(), "MARK_SPACE".to_string(), "=".to_string(), "MARK_SPACE".to_string(),
            "10".to_string(), "MARK_SPACE".to_string(), "*".to_string(), "MARK_SPACE".to_string(), "(".to_string(), "5".to_string(), "MARK_SPACE".to_string(),
            "+".to_string(), "MARK_SPACE".to_string(), "3".to_string(), ")".to_string(), ";".to_string(),
        ];
        let result = parser(line);
        assert_eq!(result, expected);
    }
}*/

