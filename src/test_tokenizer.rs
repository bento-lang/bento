use crate::token::{Position, Token};
use crate::token::TokenType::*;
use crate::tokenizer::Tokenizer;

const EXAMPLE: &'static str = r#"
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

#[test]
fn test_tokenizer() {
    let expected = vec![
        Token { token_type: Identifier("eval".parse().unwrap()), lexeme: "eval".parse().unwrap(), position: Position { pos: 1, line: 2, col: 1 } },
        Token { token_type: Assign, lexeme: ":=".parse().unwrap(), position: Position { pos: 6, line: 2, col: 6 } },
        Token { token_type: Pipe, lexeme: "|".parse().unwrap(), position: Position { pos: 9, line: 2, col: 9 } },
        Token { token_type: Identifier("expr".parse().unwrap()), lexeme: "expr".parse().unwrap(), position: Position { pos: 10, line: 2, col: 10 } },
        Token { token_type: Pipe, lexeme: "|".parse().unwrap(), position: Position { pos: 14, line: 2, col: 14 } },
        Token { token_type: LBrace, lexeme: "{".parse().unwrap(), position: Position { pos: 16, line: 2, col: 16 } },
        Token { token_type: Identifier("ops".parse().unwrap()), lexeme: "ops".parse().unwrap(), position: Position { pos: 20, line: 3, col: 3 } },
        Token { token_type: Assign, lexeme: ":=".parse().unwrap(), position: Position { pos: 24, line: 3, col: 7 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 27, line: 3, col: 10 } },
        Token { token_type: StringLiteral("+".parse().unwrap()), lexeme: "'+'".parse().unwrap(), position: Position { pos: 35, line: 4, col: 7 } },
        Token { token_type: Colon, lexeme: ":".parse().unwrap(), position: Position { pos: 38, line: 4, col: 10 } },
        Token { token_type: Identifier("_add".parse().unwrap()), lexeme: "_add".parse().unwrap(), position: Position { pos: 40, line: 4, col: 12 } },
        Token { token_type: Comma, lexeme: ",".parse().unwrap(), position: Position { pos: 44, line: 4, col: 16 } },
        Token { token_type: StringLiteral("-".parse().unwrap()), lexeme: "'-'".parse().unwrap(), position: Position { pos: 52, line: 5, col: 7 } },
        Token { token_type: Colon, lexeme: ":".parse().unwrap(), position: Position { pos: 55, line: 5, col: 10 } },
        Token { token_type: Identifier("_sub".parse().unwrap()), lexeme: "_sub".parse().unwrap(), position: Position { pos: 57, line: 5, col: 12 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 64, line: 6, col: 3 } },
        Token { token_type: Identifier("stk".parse().unwrap()), lexeme: "stk".parse().unwrap(), position: Position { pos: 68, line: 7, col: 3 } },
        Token { token_type: Assign, lexeme: ":=".parse().unwrap(), position: Position { pos: 72, line: 7, col: 7 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 75, line: 7, col: 10 } },
        Token { token_type: Comma, lexeme: ",".parse().unwrap(), position: Position { pos: 76, line: 7, col: 11 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 77, line: 7, col: 12 } },
        Token { token_type: Identifier("for".parse().unwrap()), lexeme: "for".parse().unwrap(), position: Position { pos: 81, line: 8, col: 3 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 84, line: 8, col: 6 } },
        Token { token_type: Identifier("expr".parse().unwrap()), lexeme: "expr".parse().unwrap(), position: Position { pos: 85, line: 8, col: 7 } },
        Token { token_type: Slash, lexeme: "/".parse().unwrap(), position: Position { pos: 90, line: 8, col: 12 } },
        Token { token_type: StringLiteral(" ".parse().unwrap()), lexeme: "' '".parse().unwrap(), position: Position { pos: 92, line: 8, col: 14 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 95, line: 8, col: 17 } },
        Token { token_type: Pipe, lexeme: "|".parse().unwrap(), position: Position { pos: 97, line: 8, col: 19 } },
        Token { token_type: Identifier("t".parse().unwrap()), lexeme: "t".parse().unwrap(), position: Position { pos: 98, line: 8, col: 20 } },
        Token { token_type: Pipe, lexeme: "|".parse().unwrap(), position: Position { pos: 99, line: 8, col: 21 } },
        Token { token_type: LBrace, lexeme: "{".parse().unwrap(), position: Position { pos: 101, line: 8, col: 23 } },
        Token { token_type: If, lexeme: "if".parse().unwrap(), position: Position { pos: 107, line: 9, col: 5 } },
        Token { token_type: Identifier("ops".parse().unwrap()), lexeme: "ops".parse().unwrap(), position: Position { pos: 110, line: 9, col: 8 } },
        Token { token_type: Dot, lexeme: ".".parse().unwrap(), position: Position { pos: 113, line: 9, col: 11 } },
        Token { token_type: Identifier("has".parse().unwrap()), lexeme: "has".parse().unwrap(), position: Position { pos: 114, line: 9, col: 12 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 117, line: 9, col: 15 } },
        Token { token_type: Identifier("t".parse().unwrap()), lexeme: "t".parse().unwrap(), position: Position { pos: 118, line: 9, col: 16 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 119, line: 9, col: 17 } },
        Token { token_type: Then, lexeme: "then".parse().unwrap(), position: Position { pos: 121, line: 9, col: 19 } },
        Token { token_type: LBrace, lexeme: "{".parse().unwrap(), position: Position { pos: 126, line: 9, col: 24 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 134, line: 10, col: 7 } },
        Token { token_type: Identifier("b".parse().unwrap()), lexeme: "b".parse().unwrap(), position: Position { pos: 135, line: 10, col: 8 } },
        Token { token_type: Comma, lexeme: ",".parse().unwrap(), position: Position { pos: 136, line: 10, col: 9 } },
        Token { token_type: Identifier("a".parse().unwrap()), lexeme: "a".parse().unwrap(), position: Position { pos: 138, line: 10, col: 11 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 139, line: 10, col: 12 } },
        Token { token_type: Assign, lexeme: ":=".parse().unwrap(), position: Position { pos: 141, line: 10, col: 14 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 152, line: 11, col: 9 } },
        Token { token_type: Identifier("stk".parse().unwrap()), lexeme: "stk".parse().unwrap(), position: Position { pos: 153, line: 11, col: 10 } },
        Token { token_type: Dot, lexeme: ".".parse().unwrap(), position: Position { pos: 156, line: 11, col: 13 } },
        Token { token_type: Identifier("pop".parse().unwrap()), lexeme: "pop".parse().unwrap(), position: Position { pos: 157, line: 11, col: 14 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 160, line: 11, col: 17 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 161, line: 11, col: 18 } },
        Token { token_type: Comma, lexeme: ",".parse().unwrap(), position: Position { pos: 162, line: 11, col: 19 } },
        Token { token_type: Identifier("stk".parse().unwrap()), lexeme: "stk".parse().unwrap(), position: Position { pos: 164, line: 11, col: 21 } },
        Token { token_type: Dot, lexeme: ".".parse().unwrap(), position: Position { pos: 167, line: 11, col: 24 } },
        Token { token_type: Identifier("pop".parse().unwrap()), lexeme: "pop".parse().unwrap(), position: Position { pos: 168, line: 11, col: 25 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 171, line: 11, col: 28 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 172, line: 11, col: 29 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 173, line: 11, col: 30 } },
        Token { token_type: Identifier("stk".parse().unwrap()), lexeme: "stk".parse().unwrap(), position: Position { pos: 181, line: 12, col: 7 } },
        Token { token_type: Dot, lexeme: ".".parse().unwrap(), position: Position { pos: 184, line: 12, col: 10 } },
        Token { token_type: Identifier("push".parse().unwrap()), lexeme: "push".parse().unwrap(), position: Position { pos: 185, line: 12, col: 11 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 189, line: 12, col: 15 } },
        Token { token_type: Identifier("ops".parse().unwrap()), lexeme: "ops".parse().unwrap(), position: Position { pos: 190, line: 12, col: 16 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 193, line: 12, col: 19 } },
        Token { token_type: Identifier("t".parse().unwrap()), lexeme: "t".parse().unwrap(), position: Position { pos: 194, line: 12, col: 20 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 195, line: 12, col: 21 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 196, line: 12, col: 22 } },
        Token { token_type: Identifier("a".parse().unwrap()), lexeme: "a".parse().unwrap(), position: Position { pos: 197, line: 12, col: 23 } },
        Token { token_type: Comma, lexeme: ",".parse().unwrap(), position: Position { pos: 198, line: 12, col: 24 } },
        Token { token_type: Identifier("b".parse().unwrap()), lexeme: "b".parse().unwrap(), position: Position { pos: 200, line: 12, col: 26 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 201, line: 12, col: 27 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 202, line: 12, col: 28 } },
        Token { token_type: RBrace, lexeme: "}".parse().unwrap(), position: Position { pos: 208, line: 13, col: 5 } },
        Token { token_type: Else, lexeme: "else".parse().unwrap(), position: Position { pos: 210, line: 13, col: 7 } },
        Token { token_type: Identifier("stk".parse().unwrap()), lexeme: "stk".parse().unwrap(), position: Position { pos: 215, line: 13, col: 12 } },
        Token { token_type: Dot, lexeme: ".".parse().unwrap(), position: Position { pos: 218, line: 13, col: 15 } },
        Token { token_type: Identifier("push".parse().unwrap()), lexeme: "push".parse().unwrap(), position: Position { pos: 219, line: 13, col: 16 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 223, line: 13, col: 20 } },
        Token { token_type: Identifier("num".parse().unwrap()), lexeme: "num".parse().unwrap(), position: Position { pos: 224, line: 13, col: 21 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 227, line: 13, col: 24 } },
        Token { token_type: Identifier("t".parse().unwrap()), lexeme: "t".parse().unwrap(), position: Position { pos: 228, line: 13, col: 25 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 229, line: 13, col: 26 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 230, line: 13, col: 27 } },
        Token { token_type: RBrace, lexeme: "}".parse().unwrap(), position: Position { pos: 234, line: 14, col: 3 } },
        Token { token_type: Identifier("stk".parse().unwrap()), lexeme: "stk".parse().unwrap(), position: Position { pos: 238, line: 15, col: 3 } },
        Token { token_type: Dot, lexeme: ".".parse().unwrap(), position: Position { pos: 241, line: 15, col: 6 } },
        Token { token_type: Identifier("pop".parse().unwrap()), lexeme: "pop".parse().unwrap(), position: Position { pos: 242, line: 15, col: 7 } },
        Token { token_type: LParen, lexeme: "(".parse().unwrap(), position: Position { pos: 245, line: 15, col: 10 } },
        Token { token_type: RParen, lexeme: ")".parse().unwrap(), position: Position { pos: 246, line: 15, col: 11 } },
        Token { token_type: RBrace, lexeme: "}".parse().unwrap(), position: Position { pos: 248, line: 16, col: 1 } },
        Token { token_type: EOF, lexeme: "".parse().unwrap(), position: Position { pos: 248, line: 16, col: 1 } }
    ];
    let source = String::from(EXAMPLE);
    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize();

    for (i, token) in tokens.iter().enumerate() {
        assert_eq!(token.token_type, expected[i].token_type, "Token type mismatch at index {}", i);
        assert_eq!(token.lexeme, expected[i].lexeme, "Lexeme mismatch at index {}", i);
        assert_eq!(token.position.pos, expected[i].position.pos, "Position mismatch at index {}", i);
        assert_eq!(token.position.line, expected[i].position.line, "Line mismatch at index {}", i);
        assert_eq!(token.position.col, expected[i].position.col, "Column mismatch at index {}", i);
    }
}