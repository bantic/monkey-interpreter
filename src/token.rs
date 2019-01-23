#[derive(Debug, PartialEq)]
pub enum Token<'a> {
  Illegal,
  Eof,
  Ident(&'a str),
  Int(i64),
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
  Debug,
}

pub fn lookup_ident(str: &str) -> Token {
  match str {
    "fn" => Token::Function,
    "let" => Token::Let,
    "if" => Token::If,
    "true" => Token::True,
    "false" => Token::False,
    "else" => Token::Else,
    "return" => Token::Return,

    _ => Token::Ident(str),
  }
}
