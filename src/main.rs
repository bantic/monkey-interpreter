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
