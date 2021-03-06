mod lexer;
mod parser;
mod token;

use std::io;

fn main() {
  // get stdio
  let repl = io::stdin();
  println!("Welcome to Monkey! Write code.");
  loop {
    let mut line = String::new();
    repl.read_line(&mut line).expect("Failed to read line");
    let lex = lexer::Lexer::new(&line);
    let parser = parser::Parser::new(&lex);
    println!("AST: {:?}", parser.parse());
    println!("errors: {:?}", parser.errors);
    let lex = lexer::Lexer::new(&line);
    println!("Tokens:");
    loop {
      let tok = lex.next_token();
      println!("{:?}", tok);
      if tok == token::Token::Eof {
        break;
      }
    }
  }
}
