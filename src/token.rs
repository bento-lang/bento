use std::hash::Hash;
use logos::{Logos, Span};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("if")]
    If,
    #[token("then")]
    Then,
    #[token("else")]
    Else,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("while")]
    While,
    #[token("match")]
    Match,

    #[token(":=")]
    Assign,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("|")]
    Pipe,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(".")]
    Dot,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("==")]
    Equality,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessThanEqual,
    #[token(">=")]
    GreaterThanEqual,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex(r#""[^"]*"|'[^']*'"#, |lex| lex.slice().to_string())]
    StringLiteral(String),
    #[regex(r#"(?:\d[\d_]*)(?:\.?(\d|_)*)"#, |lex| lex.slice().replace("_", "").parse::<f64>().ok())]
    Number(f64),
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("nil")]
    Nil,
    Error,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Not => write!(f, "not"),
            Token::While => write!(f, "while"),
            Token::Match => write!(f, "match"),
            Token::Assign => write!(f, ":="),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Pipe => write!(f, "|"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Dot => write!(f, "."),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Equality => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::LessThan => write!(f, "<"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThanEqual => write!(f, "<="),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::StringLiteral(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Nil => write!(f, "nil"),
            Token::Error => write!(f, "error"),
        }
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

pub type SpannedToken = Spanned<Token>;

impl SpannedToken {
    pub fn new(token: Token, span: Span) -> Self {
        SpannedToken { node: token, span }
    }
}