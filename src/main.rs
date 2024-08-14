mod token;
mod ast;
mod parser;
mod eval;
mod obj;
mod profile;

use logos::Logos;
use token::Token;
use crate::token::SpannedToken;

const SOURCE: &str = r#"
eval := |expr| {
  ops := (
      '+': _add,
      '-': _sub
  )
  stk := (,)
  for(expr / ' ') |t| {
    if ops.has(t) then {
      (b, a) :=
        (stk.pop(), stk.pop())
      stk.push(ops(t)(a, b))
    } else stk.push(num(t))
  }
  stk.pop()
}"#;

fn main() {
    let lex = Token::lexer(SOURCE);
    let spanned_tokens: Vec<SpannedToken> = lex
        .spanned()
        .map(|(token, span)| match token {
            Ok(t) => SpannedToken::new(t, span),
            Err(_) => SpannedToken::new(Token::Error, span),
        })
        .collect();

    let mut parser = parser::Parser::new(spanned_tokens.as_slice());
    let expr = parser.parse();
    println!("{:#?}", expr);
}