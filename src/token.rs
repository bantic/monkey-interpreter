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
  r#let,
  debug,
}

pub fn lookup_ident(str: &str) -> Token {
  match str {
    "fn" => Token::function,
    "let" => Token::r#let,
    _ => Token::ident(str),
  }
}

// // let x = fn a();
// struct Token<'a> {
//   Type: TokenType<'a>,
//   Literal: String,
//
