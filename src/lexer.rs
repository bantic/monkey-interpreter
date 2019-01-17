use super::token::Token;

pub struct Lexer<'a> {
  input: &'a str,
  pos: usize,
  read_pos: usize,
  pub ch: char,
}

impl<'a> Lexer<'a> {
  pub fn new(inp: &'a String) -> Lexer<'a> {
    let c = inp[0..].chars().next().unwrap();

    Lexer {
      input: &inp.as_str(),
      pos: 0,
      read_pos: c.len_utf8(),
      ch: c,
    }
  }

  pub fn read_char(&mut self) {
    if self.read_pos >= self.input.len() {
      self.ch = 0 as char;
    } else {
      let ch = self.input[self.read_pos..].chars().next().unwrap();
      self.ch = ch;
      self.pos = self.read_pos;
      self.read_pos = self.read_pos + ch.len_utf8();
    }
  }

  pub fn next_token(&mut self) -> Token {
    let t = match self.ch {
      '=' => Token::assign,
      '+' => Token::plus,
      ',' => Token::comma,
      ';' => Token::semicolon,
      '(' => Token::lparen,
      ')' => Token::rparen,
      '{' => Token::lbrace,
      '}' => Token::rbrace,
      _ => {
        if is_valid_letter(self.ch) {
          let i = self.read_identifier();
          Token::ident(i)
        } else {
          Token::illegal
        }
      }
    };

    self.read_char();
    t
  }

  // fn read_identifier(&'a mut self) -> &'a str {
  fn read_identifier(&mut self) -> &str {
    let lo = self.pos;
    while is_valid_letter(self.ch) {
      self.read_char();
    }
    &self.input[lo..self.pos]
  }
}

fn is_valid_letter(c: char) -> bool {
  c.is_alphabetic() || c == '_'
}
