use super::token::Token;
use std::cell::Cell;

pub struct Lexer<'a> {
  input: &'a str,
  pos: Cell<usize>,
  read_pos: Cell<usize>,
  pub ch: Cell<char>,
}

impl<'a> Lexer<'a> {
  pub fn new(inp: &'a String) -> Lexer<'a> {
    let c = inp[0..].chars().next().unwrap();

    Lexer {
      input: &inp.as_str(),
      pos: Cell::new(0),
      read_pos: Cell::new(c.len_utf8()),
      ch: Cell::new(c),
    }
  }

  pub fn read_char(&self) {
    if self.read_pos.get() >= self.input.len() {
      self.ch.set(0 as char);
    } else {
      let ch = self.input[self.read_pos.get()..].chars().next().unwrap();
      self.ch.set(ch);
      self.pos.set(self.read_pos.get());
      self.read_pos.set(self.read_pos.get() + ch.len_utf8());
    }
  }

  pub fn next_token(&self) -> Token {
    let t = match self.ch.get() {
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
          Token::ident(self.read_identifier())
        } else {
          Token::illegal
        }
      }
    };

    self.read_char();
    t
  }

  // fn read_identifier(&'a mut self) -> &'a str {
  fn read_identifier(&self) -> &str {
    let lo = self.pos.get();
    while is_valid_letter(self.ch.get()) {
      self.read_char();
    }
    &self.input[lo..self.pos.get()]
  }
}

fn is_valid_letter(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}
