use crate::token::{Position, Token, TokenType};
use crate::token::TokenType::*;

pub struct Tokenizer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: u32,
    col: u32,
    start_col: u32
}

impl Tokenizer {
    pub fn new(source: String) -> Tokenizer {
        Tokenizer {
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
            col: 1,
            start_col: 1
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    
    fn get_position(&self) -> Position {
        Position::new(self.start, self.line, self.start_col)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.start_col = self.col;
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, String::from(""), self.get_position()));
        self.tokens.clone()
    }
    
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        self.col += 1;
        c
    }
    
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        let position = self.get_position();
        self.tokens.push(Token::new(token_type, lexeme, position));
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        
        self.current += 1;
        self.col += 1;
        true
    }
    
    fn string(&mut self) {
        while !self.is_at_end() && self.source.chars().nth(self.current).unwrap() != '\'' {
            if self.source.chars().nth(self.current).unwrap() == '\n' {
                self.line += 1;
                self.col = 1;
            }
            
            self.advance();
        }
        
        if self.is_at_end() {
            println!("Unterminated string at line {}", self.line);
            return;
        }
        
        self.advance();
        
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(StringLiteral(value));
    }
    
    fn number(&mut self) {
        while !self.is_at_end() && self.source.chars().nth(self.current).unwrap().is_digit(10) {
            self.advance();
        }
        
        if !self.is_at_end() && self.source.chars().nth(self.current).unwrap() == '.' {
            self.advance();
            
            while !self.is_at_end() && self.source.chars().nth(self.current).unwrap().is_digit(10) {
                self.advance();
            }
        }
        
        let value = self.source[self.start..self.current].replace("_", "").parse::<f64>().unwrap();
        self.add_token(Number(value));
    }
    
    fn identifier(&mut self) {
        while !self.is_at_end() && (self.source.chars().nth(self.current).unwrap().is_alphanumeric() 
            || self.source.chars().nth(self.current).unwrap() == '_') {
            self.advance();
        }
        
        let value = self.source[self.start..self.current].to_string();
        let token_type = match value.as_str() {
            "if" => If,
            "then" => Then,
            "else" => Else,
            "and" => And,
            "or" => Or,
            "not" => Not,
            "while" => While,
            "match" => Match,
            "true" => Boolean(true),
            "false" => Boolean(false),
            "nil" => Nil,
            _ => Identifier(value)
        };
        
        self.add_token(token_type);
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(LParen),
            ')' => self.add_token(RParen),
            '{' => self.add_token(LBrace),
            '}' => self.add_token(RBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '+' => self.add_token(Plus),
            '-' => self.add_token(Minus),
            '*' => self.add_token(Star),
            '/' => self.add_token(Slash),
            '%' => self.add_token(Percent),
            ':' => {
                if self.match_char('=') {
                    self.add_token(Assign);
                } else {
                    self.add_token(Colon);
                }
            },
            '|' => self.add_token(Pipe),
            '=' => {
                if self.match_char('=') {
                    self.add_token(Equality);
                } else {
                    println!("Unexpected character: {}", c);
                }
            },
            '!' => {
                if self.match_char('=') {
                    self.add_token(NotEqual);
                } else {
                    self.add_token(Not);
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(LessThanEqual);
                } else {
                    self.add_token(LessThan);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(GreaterThanEqual);
                } else {
                    self.add_token(GreaterThan);
                }
            },
            '\'' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
                self.col = 1;
            },
            _ => println!("Unexpected character: {}", c)
        }
    }
}