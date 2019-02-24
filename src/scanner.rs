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

    loop {
        let ch = iter.next().unwrap();
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
                            name.push(iter.next().unwrap());
                        }
                        _ => break,
                    }
                }
                tokens.push(match name {
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
                            number.push(iter.next().unwrap());
                        }
                        _ => {
                            break;
                        }
                    }
                }
                tokens.push(Token::INTEGER(number as u32));
            }

            '"' => {
                let mut name = String::new();
                loop {
                    match iter.peek() {
                        Some('a'...'z') | Some('A'...'Z') | Some('_') => {
                            name.push(iter.next().unwrap());
                        }
                        '"' => {
                            iter.next();
                            break;
                        }
                    }
                }
                tokens.push(Token::STRING(name));
            }

            "!" => {
                iter.next();
                tokens.push(match iter.peek() {
                    Some('=') => Token::BANG_EQUAL,
                    Some('_') => Token::BANG,
                })
            }
            '=' => {
                iter.next();
                tokens.push(match iter.peek() {
                    Some('=') => Token::EQUAL_EQUAL,
                    Some('_') => Token::BANG_EQUAL,
                });
            }
            '<' => {
                iter.next();
                tokens.push(match iter.peek() {
                    Some('=') => Token::LESS_EQUAL,
                    Some('_') => Token::LESS,
                });
            }
            '>' => {
                iter.next();
                tokens.push(match iter.peek() {
                    Some('=') => Token::GREATER_EQUAL,
                    Some('_') => Token::GREATER,
                });
            }
            '/' => match iter.peek() {
                Some('/') => {
                    while iter.peek() != Some('\n') {
                        iter.next();
                    }
                }
                Some('*') => {
                    while iter.peek() != Some('*') {
                        iter.next();
                    }
                    iter.next();
                }
                Some('_') => tokens.push(Token::SLASH),
            },
            ' ' | '\n' => continue,
            c @ _ => {
                return Err(format!("Unexpected token: {}", ch));
            }
        };
    }
    Ok(tokens)
}
