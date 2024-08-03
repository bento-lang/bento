mod token;

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

    for spanned_token in spanned_tokens {
        match spanned_token.node {
            Token::Identifier(name) => println!("Identifier '{}' at {:?}", name, spanned_token.span),
            Token::Number(value) => println!("Number {} at {:?}", value, spanned_token.span),
            _ => println!("Other token {:?} at {:?}", spanned_token.node, spanned_token.span),
        }
    }
}