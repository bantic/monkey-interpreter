use super::token::{lookup_ident, Token};
use std::cell::Cell;

#[derive(Debug)]
pub struct Lexer<'a> {
  input: &'a str,
  pos: Cell<usize>,
  read_pos: Cell<usize>,
  pub ch: Cell<char>,
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

  pub fn next_token(&self) -> Token {
    self.skip_whitespace();

    let tok = match self.ch.get() {
      '=' => Token::assign,
      '+' => Token::plus,
      ',' => Token::comma,
      ';' => Token::semicolon,
      '(' => Token::lparen,
      ')' => Token::rparen,
      '{' => Token::lbrace,
      '}' => Token::rbrace,
      _ => {
        if is_valid_letter(self.ch.get()) {
          lookup_ident(self.read_identifier())
        } else if self.ch.get().is_numeric() {
          Token::int(self.read_number())
        } else {
          Token::illegal
        }
      }
    };

    self.read_char();
    tok
  }

  fn read_number(&self) -> i64 {
    let mut n = 0 as i64;
    while self.ch.get().is_numeric() {
      n = n * 10 + (self.ch.get().to_digit(10).unwrap() as i64);
      self.read_char();
    }
    return n;
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
    while self.ch.get().is_whitespace() {
      self.read_char();
    }
  }
}

fn is_valid_letter(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
  use super::Lexer;
  use super::Token;
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn check_lex() {
    let test = String::from("afoo");
    let lexer = Lexer::new(&test);
    assert_eq!(lexer.ch.get(), 'a');
  }

  #[test]
  fn check_lex_next_token() {
    let test = String::from("+=+=");
    let lexer = Lexer::new(&test);
    let t = lexer.next_token();
    assert_eq!(t, Token::plus);

    let t = lexer.next_token();
    assert_eq!(t, Token::assign);
  }
  #[test]
  fn check_skip_whitespace() {
    let test = "            fn";
    let lexer = Lexer::new(test);
    let tok = lexer.next_token();
    assert_eq!(tok, Token::function);
  }
  #[test]
  fn check_read_number() {
    let lexer = Lexer::new("12345");
    let tok = lexer.next_token();
    assert_eq!(tok, Token::int(12345));
  }
}
