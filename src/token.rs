#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Illegal,
  Eof,
  Ident(&'a str),
  Int(i64),
  Assign,
  Plus,
  Comma,
  Semicolon,
  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Function,
  Let,
  Debug,
}

pub fn lookup_ident(str: &str) -> Token {
  match str {
    "fn" => Token::Function,
    "let" => Token::Let,
    _ => Token::Ident(str),
  }
}
