use super::lexer;
use super::token;

// Node
//
// Program
//  - statements: Vec<StatementNode>
//
//  ExpressionNode
//
// pub struct program {
//
// }

pub enum Operator {
  Minus,
  Plus,
}

pub enum Expression {
  Literal(i32),
  Binary(Box<Expression>, Operator, Box<Expression>),
  Unary(Operator, Box<Expression>),
}

pub enum Statement {
  Let(String, Expression),
}

//pub struct program {
//  stmts: Vec<syntax>,
//}

pub struct Program(Vec<Statement>);
// pub struct stmt(Vec<syntax>);

// #[derive(Debug)]
pub struct Parser<'a> {
  lex: &'a lexer::Lexer<'a>,
  cur: token::Token<'a>,
  peek: token::Token<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lex: &'a lexer::Lexer) -> Parser<'a> {
    let cur = lex.next_token();
    let peek = lex.next_token();
    Parser {
      lex,
      cur: cur,
      peek: peek,
    }
  }

  pub fn next_token(&mut self) {
    self.cur = self.peek;
    self.peek = self.lex.next_token();
  }
}

// trait Node<'a> {
//   fn literal() -> token::Token<'a>;
// }

// trait Statement<'a> {
//   fn literal() -> token::Token<'a>;
//   fn expression();
// }

// struct Program<e: Statement<'a>> {
//   statements: 'a Vec<T>,
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_it() {
    assert_eq!(
      parse("let x = 5;"),
      Program(vec![Statement::Let(
        "x".to_string(),
        Expression::Literal(5)
      ),])
    );
  }
}
