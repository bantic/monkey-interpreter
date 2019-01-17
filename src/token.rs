#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  illegal,
  eof,
  ident(&'a str),
  int(i64),
  assign,
  plus,
  comma,
  semicolon,
  lparen,
  rparen,
  lbrace,
  rbrace,
  function,
  var,
  debug,
}

// // let x = fn a();
// struct Token<'a> {
//   Type: TokenType<'a>,
//   Literal: String,
// }
