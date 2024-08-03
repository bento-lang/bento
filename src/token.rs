#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub(crate) pos: usize,
    pub(crate) line: u32,
    pub(crate) col: u32
}

impl Position {
    pub fn new(pos: usize, line: u32, col: u32) -> Position {
        Position {
            pos,
            line,
            col
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    If, Then, Else, And, Or, Not, While, Match,
    
    Assign, Colon, Comma, Pipe, LParen, RParen, LBrace, RBrace, Dot,
    Plus, Minus, Star, Slash, Percent, Equality, NotEqual, LessThan, GreaterThan, LessThanEqual, GreaterThanEqual,
    
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    Boolean(bool),
    Nil,
    
    EOF
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) position: Position
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, position: Position) -> Token {
        Token {
            token_type,
            lexeme,
            position
        }
    }
}