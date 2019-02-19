use TokenType;
use std::collections::HashMap;
use Token;

pub struct Scanner {
    source: String,
    tokens: Vec<TokenType>,
    start: u32,
    current: u32,
    line: u32
    keywords: HashMap<&str, ToKenType>
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let keywords = HashMap::new();

        keywords.insert("and" , TokenType::AND);
        keywords.insert("class", TokenType::CLASS);
        keywords.insert("else", TokenType::ELSE);
        keywords.insert("false", TokenType::FALSE);
        keywords.insert("for", TokenType::FOR);
        keywords.insert("fun", TokenType::FUN);
        keywords.insert("if", TokenType::IF);
        keywords.insert("nil", TokenType::NIL);
        keywords.insert("or", TokenType::OR);
        keywords.insert("print", TokenType::PRINT);
        keywords.insert("return", TokenType::RETURN);
        keywords.insert("super",  TokenType::SUPER);
        keywords.insert("this", TokenType::THIS);
        keywords.insert("true",  TokenType::TRUE),
        keywords.insert("var", TokenType::VAR);
        keywords.insert("while", TokenType::WHILE);

        Scanner {
            source,
            tokens: vec![]
            start: 0,
            current: 0,
            line 1,
            keywords
        }
    }

    fn scan_tokens(&mut self) -> Vec<TokenType> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, "", hashMap::new(), self.line));
        tokens
    }

    fn scan_token()  {
        let c = self.advance();
        match c {
            Some('(') => add_token(TokenType::LEFT_PAREN),
            Some(')') => add_token(TokenType::RIGHT_PAREN),
            Some('{') => add_token(TokenType::LEFT_BRACE),
            Some('}') => add_token(TokenType::RIGHT_BRACE),
            Some(',') => add_token(TokenType::COMMA),
            Some('.') => add_token(TokenType::DOT),
            Some('-') => add_token(TokenType::MINUS),
            Some('+') => add_token(TokenType::PLUS),
            Some(';') => add_token(TokenType::SEMICOLON),
            Some('*') => add_token(TokenType::STAR),
            Some('!') => {
                if self._match('=') {
                    add_token(TokenType::BANG_EQUAL);
                } else {
                    add_token(TokenType::BANG);
                }
            },
            Some('=') => {
                if self._match('=') {
                    add_token(TokenType::EQUAL_EQUAL);
                } else {
                    add_token(TokenType::BANG_EQUAL);
                }
            }
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let res = self.source.get(self.current - 1)
        self.current += 1;
        res
    }

    fn _match (expected_char: char) -> bool {
         if self.is_end() {
             return false;
         }
         if self.source.get(self.current) !== Some(expected_char) {
             return false;
         }

        self.current += 1;
        return true;
    }

    fn add_token (&mut self, token: TokenType, text: Option<&str>) {
        let text = &self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token::new(token, text, HashMap::new(), self.line));
    }


}
