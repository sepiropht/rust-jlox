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
            },
            Some('<') => {
                if self._match('=') {
                    add_token(TokenType::LESS_EQUAL);
                } else {
                    add_token(TokenType::LESS);
                }

            },
            Some('>') => {
                if self._match('=') {
                    add_token(TokenType::GREATER_EQUAL);
                } else {
                    add_token(TokenType::GREATER);
                }

            },
            Some('/') => {
                if self._match('/') {
                    while self.peek !== '\n' && !self.is_end() {
                        self.advance();
                    }
                } else if self._match('*') {
                    self.scan_block_comment();
                } else {
                    self.add_token(TokenType::SLASH);
                }
            },
            Some(' ') || Some('\r') || Some('\t') => (),
            Some('\n') => self.line += 1;
            Some '"' => self.scan_string(),
             _ => if c.is_numeric() {
                 self.scanNumber();
             } else if c.isphanumeric() {
                 self.scan_identifier();
             } else {
                 panic!("unexpected caracter");
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

    fn scan_identifier(&self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.source.get(self.start, self.current).unwrap();
        const type = text in self.keywords ? self.keywords[text] : TokenType::IDENTIFIER;
        self.add_token(type);
    }

    fn peek(&mut self) -> char {
        if self.is_end() {
            return '\0';
        }
        self.source.get(self.current).unwrap()

    }

    fn scan_string(&self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peel() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            panic!(self.line, "Unterminated string.");
        }

        self.advance();

        let value = self.source.get(self.start + 1, self.curent -1);
        self.add_token(TokenType::STRING, value);
    }

    fn scan_number(&self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek == '.' && self.peek_next().is_numeric() {
            self.advance();
        }

        while self.peek().is_numeric() {
            self.advance();
        }

        self.add_token(TokenType::NUMBER, self.source.get(self.start, self.current));

    }

    fn peekNext() {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.get(current + 1).unwrap();

    }


}
