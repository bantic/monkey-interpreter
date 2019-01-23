mod lexer;
mod token;

use std::io;

fn main() {
  // get stdio
  let repl = io::stdin();
  loop {
    let mut line = String::new();
    repl.read_line(&mut line).expect("Failed to read line");
    println!("got line: {}", line);
    let lex = lexer::Lexer::new(&line);
    let t = lex.next_token();
    println!("got t: {:?}", t);
  }
}
