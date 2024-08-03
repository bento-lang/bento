mod token;

use logos::Logos;
use token::Token;

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
    let lexer = Token::lexer(SOURCE);
    for token in lexer {
        println!("{:?}", token);
    }
}