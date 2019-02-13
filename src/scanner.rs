use token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.finished_scan() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "".to_string(),
            self.line,
        ));

        vec![]
    }

    fn finished_scan(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let token_type = match self.advance() {
            '(' => Some(TokenType::LEFT_PAREN),
            ')' => Some(TokenType::RIGHT_PAREN),
            '{' => Some(TokenType::LEFT_BRACE),
            '}' => Some(TokenType::RIGHT_BRACE),
            ',' => Some(TokenType::COMMA),
            '.' => Some(TokenType::DOT),
            '-' => Some(TokenType::MINUS),
            '+' => Some(TokenType::PLUS),
            ';' => Some(TokenType::SEMICOLON),
            '*' => Some(TokenType::STAR),
            '!' => {
                if self.is_next('=') {
                    Some(TokenType::BANG_EQUAL)
                } else {
                    TokenType::BANG
                }
            }
            '=' => {
                if self.is_next('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                }
            }
            '<' => {
                if self.is_next('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                }
            }
            '>' => {
                if self.is_next('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                }
            }
            '/' => {
                if self.is_next('/') {
                    while !self.finished_scan() && self.peek() != '\n' {
                        self.advance()
                    }

                    None
                } else {
                    Some(TokenType::SLASH)
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;

                None
            }
            '"' => {
                while !self.finished_scan() && self.peek() != '"' {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }

                    self.advance();
                }

                Some(TokenType::STRING)
            }
            _ => {
                println!("Unexpected character at line: {}", self.line);

                None
            }
        };

        if let Some(token_type) = token_type {
            self.tokens.push(Token::new(
                token_type,
                self.source[self.start..self.current],
                "".to_string(),
                self.line,
            ));
        }
    }

    fn is_next(&mut self, expected: char) -> bool {
        if self.finished_scan() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;

            true
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.finished_scan() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }
}
