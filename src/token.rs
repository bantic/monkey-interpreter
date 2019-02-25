#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
  Illegal,
  Eof,
  Ident,
  Int,
  Assign,
  Eq,
  NotEq,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,
  Lt,
  Gt,
  Comma,
  Semicolon,
  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Function,
  Let,
  If,
  Else,
  True,
  False,
  Return,
  #[allow(dead_code)]
  Debug,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'a> {
  pub kind: TokenKind,
  pub literal: &'a str,
}

pub fn lookup_kind(str: &str) -> TokenKind {
  use TokenKind::*;
  match str {
    "fn" => Function,
    "let" => Let,
    "if" => If,
    "true" => True,
    "false" => False,
    "else" => Else,
    "return" => Return,

    _ => Ident,
  }
}
