use std::collections::HashMap;
use TokenType;

pub struct Token {
    token_type : TokenType,
    lexeme: String,
    literal: HashMap,
    line: u32
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl ToString for token {
    fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
