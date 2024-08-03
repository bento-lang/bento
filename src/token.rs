use logos::{Logos, Span};

#[derive(Debug, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Logos, Debug, PartialEq)]
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

pub type SpannedToken = Spanned<Token>;

impl SpannedToken {
    pub fn new(token: Token, span: Span) -> Self {
        SpannedToken { node: token, span }
    }
}