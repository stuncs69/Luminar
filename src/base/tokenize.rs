use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PATTERNS: Vec<(&'static str, &'static str)> = vec![
        ("KEYWORD", r#"^(if|else|while|for|function|return|var)$"#),
        ("OPERATOR", r#"^(\+|\-|\*|\/|\=)$"#),
        ("IDENTIFIER", r#"^[a-zA-Z_]\w*$"#),
        ("LITERAL", r#"^(\d+(\.\d+)?|\"(\\.|[^"])*\"|\'(\\.|[^\'])*\')$"#),
        ("SPECIAL_SYMBOL", r#"^(\(|\)|\{|\}|\[|\]|;)$"#),
        ("COMMENT", r#"^\/\/.*$"#),
        ("VAR_REF", r#"^\$[a-zA-Z_]\w*$"#),
        ("FUNCCALL", r#"[a-zA-Z_]*\w\.[a-zA-Z_]*\w"#),
        ("LOGIC", r#"^(>|<|>=|<=|==|!=)$"#),
    ];
}

pub struct TokenType {
    pub token_type: &'static str,
    pub value: String,
}

pub fn tokenize(code: &str) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let lines: Vec<&str> = code.split('\n').collect();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            for (token_type, pattern) in PATTERNS.iter() {
                let re = Regex::new(pattern).unwrap();
                if re.is_match(word) {
                    tokens.push(TokenType {
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
