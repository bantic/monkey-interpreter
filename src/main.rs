mod lexer;
mod token;

use std::io;

fn main() {
  // get stdio
  let repl = io::stdin();
  loop {
    let mut line = String::new();
    repl.read_line(&mut line).expect("Failed to read line");
    let lex = lexer::Lexer::new(&line);
    loop {
      let tok = lex.next_token();
      println!("{:?}", tok);
      if tok == token::Token::Eof {
        break;
      }
    }
  }
}
