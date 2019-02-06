use TokenType;
use std::collections::HashMap;
use Token;

pub struct Scanner {
    source: &str,
    tokens: Vec<TokenType>
    start: u32,
    current: u32,
    line: u32
}

impl Scanner {
    pub fn new(source: &str) -> Scanner{
        Scanner {
            source,
            tokens: vec![]
            0,
            0,
            1
        }
    }

    fn scan_tokens(&mut self) -> Vec<TokenType> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        tokens.push(Token::new(TokenType::EOF, "", null, self.line));
        tokens
    }

    fn scan_token(&self)

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }


}
