use std::fmt;

pub enum Expression {
    Literal(String),
    Identifier(String),
}

pub enum Statement {
    Assignment {
        identifier: String,
        expression: Expression,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Keyword,
    Operator,
    Identifier,
    Literal,
    SpecialSymbol,
    Comment,
    VarRef,
    FuncCall,
    Logic,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Keyword => write!(f, "Keyword"),
            TokenType::Operator => write!(f, "Operator"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Literal => write!(f, "Literal"),
            TokenType::SpecialSymbol => write!(f, "SpecialSymbol"),
            TokenType::Comment => write!(f, "Comment"),
            TokenType::VarRef => write!(f, "VarRef"),
            TokenType::FuncCall => write!(f, "FuncCall"),
            TokenType::Logic => write!(f, "Logic"),
        }
    }
}