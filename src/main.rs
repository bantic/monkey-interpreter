mod lexer;
mod token;

use std::io;

fn main() {
  // get stdio
  let repl = io::stdin();
  loop {
    let mut line = String::new();
    let n = match repl.read_line(&mut line) {
      Ok(n) => n,
      _ => continue,
    };
    println!("got line: {}", line);
    let lex = lexer::Lexer::new(&line);
    let t = lex.next_token();
    println!("got t: {:?}", t);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn check_lex() {
    let test = String::from("afoo");
    let lexer = super::lexer::Lexer::new(&test);
    assert_eq!(lexer.ch.get(), 'a');
  }

  #[test]
  fn check_lex_next_token() {
    let test = String::from("+=+=");
    let lexer = super::lexer::Lexer::new(&test);
    let t = lexer.next_token();
    assert_eq!(t, super::token::Token::plus);

    let t = lexer.next_token();
    assert_eq!(t, super::token::Token::assign);
  }
}
