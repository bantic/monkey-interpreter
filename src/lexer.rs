use std::cell::Cell;

use super::token::{lookup_kind, Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'a> {
  input: &'a str,
  pos: Cell<usize>,
  read_pos: Cell<usize>,
  pub ch: Cell<char>,
}

macro_rules! emit_token {
  ( ($t:expr, $s:expr) ) => {
    Token {
      kind: $t,
      literal: &$s.input[$s.pos.get()..$s.pos.get() + 1],
    }
  };
}

impl<'a> Lexer<'a> {
  pub fn new(inp: &'a str) -> Lexer<'a> {
    let c = inp[0..].chars().next().unwrap();

    Lexer {
      // input string
      input: &inp, // .as_str(),

      // position of current rune
      pos: Cell::new(0),

      // the position of next rune
      read_pos: Cell::new(c.len_utf8()),

      // the rune
      ch: Cell::new(c),
    }
  }

  pub fn read_char(&self) {
    let ch: char;
    if self.read_pos.get() >= self.input.len() {
      ch = '\0';
    } else {
      ch = self.input[self.read_pos.get()..].chars().next().unwrap();
    }
    self.ch.set(ch);
    self.pos.set(self.read_pos.get());
    self.read_pos.set(self.read_pos.get() + ch.len_utf8());
  }

  pub fn peek_char(&self) -> char {
    if self.read_pos.get() > self.input.len() {
      '\0'
    } else {
      self.input[self.read_pos.get()..].chars().next().unwrap()
    }
  }

  pub fn next_token(&self) -> Token {
    self.skip_whitespace();

    let tok = match self.ch.get() {
      '=' => {
        if self.peek_char() == '=' {
          self.read_char();
          Token {
            kind: TokenKind::Eq,
            literal: &self.input[self.pos.get() - 2..self.pos.get()],
          }
        } else {
          emit_token!((TokenKind::Assign, self))
        }
      }
      '+' => emit_token!((TokenKind::Plus, self)),
      '-' => emit_token!((TokenKind::Minus, self)),
      '!' => {
        if self.peek_char() == '=' {
          self.read_char();
          Token {
            kind: TokenKind::NotEq,
            literal: &self.input[self.pos.get() - 2..self.pos.get()],
          }
        } else {
          emit_token!((TokenKind::Bang, self))
        }
      }
      '*' => emit_token!((TokenKind::Asterisk, self)),
      '/' => emit_token!((TokenKind::Slash, self)),
      '<' => emit_token!((TokenKind::Lt, self)),
      '>' => emit_token!((TokenKind::Gt, self)),
      ',' => emit_token!((TokenKind::Comma, self)),
      ';' => emit_token!((TokenKind::Semicolon, self)),
      '(' => emit_token!((TokenKind::Lparen, self)),
      ')' => emit_token!((TokenKind::Rparen, self)),
      '{' => emit_token!((TokenKind::Lbrace, self)),
      '}' => emit_token!((TokenKind::Rbrace, self)),
      '\0' => Token {
        kind: TokenKind::Eof,
        literal: "",
      },
      _ => {
        if is_valid_letter(self.ch.get()) {
          let literal = self.read_identifier();
          return Token {
            kind: lookup_kind(literal),
            literal,
          };
        } else if self.ch.get().is_numeric() {
          return Token {
            kind: TokenKind::Int,
            literal: self.read_number(),
          };
        } else {
          emit_token!((TokenKind::Illegal, self))
        }
      }
    };

    self.read_char();
    tok
  }

  fn read_number(&self) -> &str {
    let lo = self.pos.get();
    while self.ch.get().is_numeric() {
      self.read_char();
    }
    let result = &self.input[lo..self.pos.get()]; // a..=b === [a,b]
    result
  }

  fn read_identifier(&self) -> &str {
    let lo = self.pos.get();
    while is_valid_letter(self.ch.get()) {
      self.read_char();
    }
    let result = &self.input[lo..self.pos.get()]; // a..=b === [a,b]
    result
  }

  fn skip_whitespace(&self) {
    while self.ch.get().is_whitespace() || self.ch.get() == '\n' || self.ch.get() == '\r' {
      self.read_char();
    }
  }
}

fn is_valid_letter(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
  use super::{Lexer, TokenKind};

  #[test]
  fn check_lex() {
    let test = String::from("afoo");
    let lexer = Lexer::new(&test);
    assert_eq!(lexer.ch.get(), 'a');
  }

  #[test]
  fn check_lex_next_token() {
    let test = String::from("+=+=;");
    let lexer = Lexer::new(&test);
    let mut t = lexer.next_token();

    println!("{:?}", t);
    assert_eq!(t.kind, TokenKind::Plus);

    t = lexer.next_token();
    println!("{:?}", t);

    assert_eq!(t.kind, TokenKind::Assign);
  }
  #[test]
  fn check_skip_whitespace() {
    let test = "            fn";
    let lexer = Lexer::new(test);
    let tok = lexer.next_token();
    assert_eq!(tok.kind, TokenKind::Function);
  }
  #[test]
  fn check_read_number() {
    let lexer = Lexer::new("12345");
    let tok = lexer.next_token();
    assert_eq!(tok.kind, TokenKind::Int);
    assert_eq!(tok.literal, "12345");
  }

  #[test]
  fn check_lexer_robustness() {
    let input = "let if true (9 > 16) + a =b
    10000
    5 == !x;
    6 != z;
    false fn = alphabet,<; { }
    ";

    let expect = [
      TokenKind::Let,
      TokenKind::If,
      TokenKind::True,
      TokenKind::Lparen,
      TokenKind::Int,
      TokenKind::Gt,
      TokenKind::Int,
      TokenKind::Rparen,
      TokenKind::Plus,
      TokenKind::Ident,
      TokenKind::Assign,
      TokenKind::Ident,
      TokenKind::Int,
      TokenKind::Int,
      TokenKind::Eq,
      TokenKind::Bang,
      TokenKind::Ident,
      TokenKind::Semicolon,
      TokenKind::Int,
      TokenKind::NotEq,
      TokenKind::Ident,
      TokenKind::Semicolon,
      TokenKind::False,
      TokenKind::Function,
      TokenKind::Assign,
      TokenKind::Ident,
      TokenKind::Comma,
      TokenKind::Lt,
      TokenKind::Semicolon,
      TokenKind::Lbrace,
      TokenKind::Rbrace,
      TokenKind::Eof,
    ];

    let lexer = Lexer::new(input);

    for expected_token in expect.iter() {
      let tok = lexer.next_token();
      assert_eq!(*expected_token, tok.kind);
    }
  }
}
