use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    Boolean(bool),
    Nil,
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Call(Box<Expr>, Vec<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    While(Box<Expr>, Box<Expr>),
    Property(Box<Expr>, String),
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Lambda(Vec<String>, Box<Expr>),
    Match(Box<Expr>, Vec<(Expr, Expr)>),
}