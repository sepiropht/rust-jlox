#[derive(Debug)]
pub enum Token {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(u32),
    //                     // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

fn single_token(ch: char) -> Option<Token> {
    match ch {
        '(' => Some(Token::LEFT_PAREN),
        ')' => Some(Token::RIGHT_PAREN),
        '{' => Some(Token::LEFT_BRACE),
        '}' => Some(Token::RIGHT_BRACE),
        ',' => Some(Token::COMMA),
        '.' => Some(Token::DOT),
        '-' => Some(Token::MINUS),
        '+' => Some(Token::PLUS),
        ';' => Some(Token::SEMICOLON),
        '*' => Some(Token::STAR),
        _ => None,
    }
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut iter = source.chars().peekable();
    let mut line = 1;
    loop {
        let ch = iter.next();
        if ch.is_none() {
            break;
        }
        let ch = ch.unwrap();

        if let Some(token) = single_token(ch) {
            tokens.push(token);
            continue;
        }

        match ch {
            i @ 'a'...'z' | i @ 'A'...'Z' => {
                let mut name = String::new();
                name.push(i);
                loop {
                    match iter.peek() {
                        Some('a'...'z') | Some('A'...'Z') | Some('_') => {
                            name.push(iter.next().expect("letter"));
                        }
                        _ => break,
                    }
                }
                tokens.push(match name.as_str() {
                    "and" => Token::AND,
                    "class" => Token::CLASS,
                    "else" => Token::ELSE,
                    "false" => Token::FALSE,
                    "for" => Token::FOR,
                    "fun" => Token::FUN,
                    "if" => Token::IF,
                    "nil" => Token::NIL,
                    "or" => Token::OR,
                    "print" => Token::PRINT,
                    "return" => Token::RETURN,
                    "super" => Token::SUPER,
                    "this" => Token::THIS,
                    "true" => Token::TRUE,
                    "var" => Token::VAR,
                    "while" => Token::WHILE,
                    _ => Token::IDENTIFIER(name),
                });
            }
            n @ '0'...'9' => {
                let mut number = String::new();
                number.push(n);
                loop {
                    match iter.peek() {
                        Some('0'...'9') => {
                            number.push(iter.next().expect("number"));
                        }
                        _ => {
                            break;
                        }
                    }
                }
                tokens.push(Token::NUMBER(number.parse().expect("number")));
            }

            '"' => {
                let mut name = String::new();
                loop {
                    match iter.peek() {
                        Some('"') => {
                            iter.next();
                            break;
                        }
                        _ => {
                            name.push(iter.next().expect("letter 2"));
                        }
                    }
                }
                tokens.push(Token::STRING(name));
            }

            '!' => tokens.push(match iter.peek() {
                Some('=') => {
                    iter.next();
                    Token::BANG_EQUAL
                }
                _ => panic!("no"),
            }),
            '=' => {
                tokens.push(match iter.peek() {
                    Some('=') => {
                        iter.next();
                        Token::EQUAL_EQUAL
                    }
                    _ => Token::BANG_EQUAL,
                });
            }
            '<' => {
                tokens.push(match iter.peek() {
                    Some('=') => {
                        iter.next();
                        Token::LESS_EQUAL
                    }
                    _ => Token::LESS,
                });
            }
            '>' => {
                tokens.push(match iter.peek() {
                    Some('=') => {
                        iter.next();
                        Token::GREATER_EQUAL
                    }
                    _ => Token::GREATER,
                });
            }
            '/' => match iter.peek() {
                Some('/') => {
                    while iter.peek() != Some(&'\n') {
                        iter.next().unwrap();
                    }
                }
                Some('*') => {
                    iter.next();
                    iter.peek().unwrap();
                    while iter.peek() != Some(&'*') {
                        iter.next().unwrap();
                    }
                    iter.next();
                }
                _ => tokens.push(Token::SLASH),
            },
            ' ' | '\n' => {
                if ch == '\n' {
                    line += 1;
                }
                continue
            },
            c @ _ => {
                return Err(format!("line: {}, Unexpected token: {}", line, c));
            }
        };
    }
    Ok(tokens)
}
