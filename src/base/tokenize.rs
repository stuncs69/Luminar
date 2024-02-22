use crate::base::enums;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATTERNS: Vec<(enums::TokenType, &'static str)> = vec![
        (enums::TokenType::Keyword, r#"^(if|else|while|for|function|return|var)$"#),
        (enums::TokenType::Operator, r#"^(\+|\-|\*|\/|\=)$"#),
        (enums::TokenType::Identifier, r#"^[a-zA-Z_]\w*$"#),
        (enums::TokenType::Literal, r#"^(\d+(\.\d+)?|\"(\\.|[^"])*\"|\'(\\.|[^\'])*\')$"#),
        (enums::TokenType::SpecialSymbol, r#"^(\(|\)|\{|\}|\[|\]|;)$"#),
        (enums::TokenType::Comment, r#"^\/\/.*$"#),
        (enums::TokenType::VarRef, r#"^\$[a-zA-Z_]\w*$"#),
        (enums::TokenType::FuncCall, r#"[a-zA-Z_]*\w\.[a-zA-Z_]*\w"#),
        (enums::TokenType::Logic, r#"^(>|<|>=|<=|==|!=)$"#),
    ];
}

pub fn tokenize(code: &str) -> Vec<enums::Token> {
    let mut tokens = Vec::new();
    let lines: Vec<&str> = code.split('\n').collect();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            for (token_type, pattern) in PATTERNS.iter() {
                let re = Regex::new(pattern).unwrap();
                if re.is_match(word) {
                    tokens.push(enums::Token {
                        token_type: *token_type,
                        value: word.to_string(),
                    });
                    break;
                }
            }
        }
    }

    tokens
}
