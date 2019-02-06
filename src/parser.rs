use super::lexer;
use super::token;

pub struct Ident<'a>(&'a str);

pub struct Expr {
  node: ExprKind,
}

pub enum ExprKind {
  Literal(i32),
  Ident(String),
}

struct Stmt<'a> {
  node: StmtKind<'a>,
}

struct LetStmt<'a> {
  name: Ident<'a>,
  value: Expr,
}

pub enum StmtKind<'a> {
  Let(LetStmt<'a>),
}

pub struct Program<'a>(Vec<Stmt<'a>>);

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_it() {
    assert_eq!(
      parse("let x = 5;"),
      Program(vec![Stmt {
        node: StmtKind::Let(LetStmt {
          name: Ident("x"),
          value: Expr {
            node: ExprKind::Literal(5)
          }
        })
      }])
    );
  }
}
